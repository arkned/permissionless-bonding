use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Transfer, Token};

use crate::{constant::*, states::*};

#[derive(Accounts)]
pub struct InitNewProject<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub lp_mint: Account<'info, Mint>,
    pub lp_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        seeds = [token_mint.key().as_ref(), TOKEN_VAULT_SEED.as_ref()],
        bump,
        payer = initializer,
        token::mint = token_mint,
        token::authority = vault_account,
    )]
    pub vault_account: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        seeds = [token_mint.key().as_ref(), PROJECT_INFO_SEED.as_ref()],
        bump,
        payer = initializer,
        space = 8 + 32 + 32 + 32 + 32 + 8 + 8 * 3 + 8 * 6 + 8 * 2 + 1024 // 1024 gap
    )]
    pub project_info: Box<Account<'info, ProjectInfo>>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> InitNewProject<'info> {
    pub fn into_deposit_to_vault_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self
                .token_account
                .to_account_info()
                .clone(),
            to: self.vault_account.to_account_info().clone(),
            authority: self.initializer.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        constraint = *user.key == project_info.project_owner,
    )]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [project_info.project_token.as_ref(), PROJECT_INFO_SEED.as_ref()],
        bump,
    )]
    pub project_info: Box<Account<'info, ProjectInfo>>,
}

#[derive(Accounts)]
pub struct Bond<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub lp_mint: Account<'info, Mint>,
    #[account(mut)]
    pub lp_deposit_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = project_info.lp_token_account == lp_recieve_account.key()
    )]
    pub lp_recieve_account: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub receive_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [token_mint.key().as_ref(), PROJECT_INFO_SEED.as_ref()],
        bump,
    )]
    pub project_info: Box<Account<'info, ProjectInfo>>,
    #[account(
        init_if_needed,
        seeds = [token_mint.key().as_ref(), user.key().as_ref(), BONDS_INFO_SEED.as_ref()],
        bump,
        payer = user,
        space = 256 // 16 is enough for now
    )]
    pub bonds_info: Box<Account<'info, BondsInfo>>,
    #[account(
        init,
        seeds = [token_mint.key().as_ref(), user.key().as_ref(), VESTING_INFO_SEED.as_ref(), bonds_info.total_bonds.to_string().as_bytes()],
        bump,
        payer = user,
        space = 256 // 32 is enough for now
    )]
    pub vesting_info: Box<Account<'info, VestingInfo>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>
}

impl<'info> Bond<'info> {
    pub fn into_bond_lp_to_project_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self
                .lp_deposit_account
                .to_account_info()
                .clone(),
            to: self.lp_recieve_account.to_account_info().clone(),
            authority: self.user.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}


#[derive(Accounts)]
#[instruction(bond_id: u64)]
pub struct WithdrawVesting<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub taker_receive_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [project_info.project_token.as_ref(), TOKEN_VAULT_SEED.as_ref()],
        bump,
    )]
    pub vault_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [project_info.project_token.as_ref(), PROJECT_INFO_SEED.as_ref()],
        bump,
    )]
    pub project_info: Box<Account<'info, ProjectInfo>>,
    #[account(
        seeds = [project_info.project_token.as_ref(), taker.key().as_ref(), VESTING_INFO_SEED.as_ref(), bond_id.to_string().as_bytes()],
        bump,
    )]
    pub vesting_info: Box<Account<'info, VestingInfo>>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> WithdrawVesting<'info> {
    pub fn into_transfer_to_taker(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self
                .vault_account
                .to_account_info()
                .clone(),
            to: self.taker_receive_token_account.to_account_info().clone(),
            authority: self.vault_account.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }


    pub fn taker_vested_amount(&self, accuracy: u64) -> u64 {
        let lock_end_time = self.vesting_info.start_time + self.project_info.lock_period;
        let vesting_end_time = lock_end_time + self.project_info.vesting_period;
        let vesting_info = &self.vesting_info;

        let vesting_unlock_amount =
            vesting_info.total_amount * self.project_info.instant_unlock / accuracy;

        if self.vesting_info.start_time == 0 || vesting_info.total_amount == 0 {
            return 0;
        }

        if self.clock.unix_timestamp as u64 <= lock_end_time {
            return vesting_unlock_amount;
        }

        if self.clock.unix_timestamp as u64 > vesting_end_time {
            return vesting_info.total_amount;
        }

        let initial_unlock_amount =
            vesting_info.total_amount * self.project_info.initial_unlock / accuracy;
        let unlock_amount_per_interval =
            vesting_info.total_amount * self.project_info.release_rate / accuracy;
        let mut vested_amount = (self.clock.unix_timestamp as u64 - lock_end_time)
            / self.project_info.release_interval
            * unlock_amount_per_interval
            + initial_unlock_amount
            + vesting_unlock_amount;
        let withdrawn_amount = vesting_info.withdrawn_amount;

        if withdrawn_amount > vested_amount {
            vested_amount = withdrawn_amount;
        }

        if vested_amount > vesting_info.total_amount {
            vested_amount = vesting_info.total_amount;
        }
        return vested_amount;
    }
}
