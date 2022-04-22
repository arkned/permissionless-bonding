use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::context_accounts::*;
use crate::states::*;

pub fn process_init_new_project(
    ctx: Context<InitNewProject>,
    amount: u64,
    price: u64,
    discount_settings: DiscountSettings,
    vesting_schedule: VestingSchedule
) -> Result<()> {
    ctx.accounts.project_info.project_owner = ctx.accounts.initializer.key();
    
    ctx.accounts.project_info.project_token = ctx.accounts.token_mint.key();
    ctx.accounts.project_info.lp_token = ctx.accounts.lp_mint.key();
    ctx.accounts.project_info.lp_token_account = ctx.accounts.lp_token_account.key();
    ctx.accounts.project_info.token_amount = amount;
    ctx.accounts.project_info.price = price;

    ctx.accounts.project_info.min_discout = discount_settings.min_discout;
    ctx.accounts.project_info.max_discount = discount_settings.max_discount;
    ctx.accounts.project_info.discount_mode = discount_settings.discount_mode;

    ctx.accounts.project_info.release_interval = vesting_schedule.release_interval;
    ctx.accounts.project_info.release_rate = vesting_schedule.release_rate;
    ctx.accounts.project_info.instant_unlock = vesting_schedule.instant_unlock;
    ctx.accounts.project_info.initial_unlock = vesting_schedule.initial_unlock;
    ctx.accounts.project_info.lock_period = vesting_schedule.lock_period;
    ctx.accounts.project_info.vesting_period = vesting_schedule.vesting_period;

    token::transfer(ctx.accounts.into_deposit_to_vault_context(), amount)?;
    Ok(())
}
