use anchor_lang::prelude::*;
use anchor_spl::token;
use std::convert::TryFrom;

use crate::context_accounts::*;
use crate::constant;

pub fn process_bond(
    ctx: Context<Bond>,
    lp_amount: u64,
) -> Result<()> {
    ctx.accounts.bonds_info.total_bonds = ctx.accounts.bonds_info.total_bonds + 1;

    let discount_rate; 
    match ctx.accounts.project_info.discount_mode {
        1 => {
            discount_rate = 
                ctx.accounts.project_info.min_discout + 
                (ctx.accounts.project_info.max_discount - ctx.accounts.project_info.min_discout)
                 * ctx.accounts.project_info.vested_amount
                 / ctx.accounts.project_info.token_amount;
        }
        2 => {
            discount_rate = 
                ctx.accounts.project_info.max_discount -
                (ctx.accounts.project_info.max_discount - ctx.accounts.project_info.min_discout)
                 * ctx.accounts.project_info.vested_amount
                 / ctx.accounts.project_info.token_amount;
        },
        _ => {
            discount_rate = 0;
        }
    }

    let new_price = ctx.accounts.project_info.price * (10000 - discount_rate) / 10000;
    let new_vesting_amount = (lp_amount as u128)
        .checked_mul(constant::ACCURACY as u128).unwrap()
        .checked_div(new_price as u128).unwrap()
        .checked_mul(u128::pow(10, ctx.accounts.token_mint.decimals as u32)).unwrap()
        .checked_div(u128::pow(10, ctx.accounts.lp_mint.decimals as u32)).unwrap();
    let new_vesting_amount = u64::try_from(new_vesting_amount).ok().unwrap();

    token::transfer(
        ctx.accounts.into_bond_lp_to_project_context(),
        lp_amount
    )?;

    ctx.accounts.project_info.bonded_lp_amount = ctx.accounts.project_info.bonded_lp_amount + lp_amount;
    ctx.accounts.project_info.vested_amount = ctx.accounts.project_info.vested_amount + new_vesting_amount;

    ctx.accounts.vesting_info.total_amount = new_vesting_amount;
    ctx.accounts.vesting_info.start_time = ctx.accounts.clock.unix_timestamp as u64;

    Ok(())
}
