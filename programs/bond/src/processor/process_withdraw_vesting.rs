use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::constant::{ACCURACY, TOKEN_VAULT_SEED};
use crate::context_accounts::*;

pub fn process_withdraw_vesting(
    ctx: Context<WithdrawVesting>,
    project_bonding_id: u64
) -> Result<()> {
    let vested_amount = ctx.accounts.taker_vested_amount(ACCURACY);
    let withdrawable_amount = vested_amount - ctx.accounts.vesting_info.withdrawn_amount;
    if withdrawable_amount > 0 {
        ctx.accounts.vesting_info.withdrawn_amount = vested_amount;

        let (vault_account, vault_account_bump) = Pubkey::find_program_address(&[
                ctx.accounts.project_info.project_token.as_ref(),
                TOKEN_VAULT_SEED.as_ref(),
                project_bonding_id.to_string().as_bytes(),
            ], &ctx.program_id);

        token::transfer(
            ctx.accounts.into_transfer_to_taker().with_signer(&[&[
                ctx.accounts.project_info.project_token.as_ref(),
                TOKEN_VAULT_SEED.as_ref(),
                project_bonding_id.to_string().as_bytes(),
                &[vault_account_bump],
            ]]),
            withdrawable_amount,
        )?;
    }

    Ok(())
}
