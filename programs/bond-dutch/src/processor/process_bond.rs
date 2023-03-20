use anchor_lang::prelude::*;
use anchor_spl::token;
use std::convert::TryFrom;

use crate::context_accounts::*;
use crate::constant;

pub fn process_bond(
    ctx: Context<Bond>,
    lp_amount: u64,
) -> Result<()> {
    let timed_price = 
        ctx.accounts.auction_info.min_price + 
        (ctx.accounts.auction_info.max_price - ctx.accounts.auction_info.min_price) *
        (ctx.accounts.auction_info.auction_end_time - ctx.accounts.clock.unix_timestamp as u64) / 
        (ctx.accounts.auction_info.auction_end_time - ctx.accounts.auction_info.auction_start_time);

    let cur_price = ctx.accounts.auction_info.bonded_lp_amount * constant::ACCURACY / ctx.accounts.auction_info.token_amount;

    let new_price = if timed_price > cur_price { timed_price } else  { cur_price };
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

    ctx.accounts.auction_info.bonded_lp_amount += lp_amount;

    ctx.accounts.vesting_info.bonded_lp_amount += lp_amount;

    Ok(())
}
