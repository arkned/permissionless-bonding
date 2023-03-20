use anchor_lang::prelude::*;

use crate::{context_accounts::*, constant};

pub fn process_end_auction(
    ctx: Context<EndAuction>
) -> Result<()> {
    let final_price = ctx.accounts.auction_info.bonded_lp_amount * constant::ACCURACY / ctx.accounts.auction_info.token_amount;
    ctx.accounts.auction_info.is_auction_success = final_price >= ctx.accounts.auction_info.min_price;
    ctx.accounts.auction_info.final_price = final_price;
    Ok(())
}
