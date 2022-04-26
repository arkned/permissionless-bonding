use anchor_lang::prelude::*;

use crate::context_accounts::*;

pub fn process_update_price(
    ctx: Context<UpdatePrice>,
    new_price: u64
) -> Result<()> {
    ctx.accounts.project_info.price = new_price;
    Ok(())
}
