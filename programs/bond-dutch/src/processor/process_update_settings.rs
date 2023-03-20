use anchor_lang::prelude::*;

use crate::{context_accounts::*, states::{AuctionSettings, VestingSchedule}};

pub fn process_update_settings(
    ctx: Context<UpdateSettings>,
    auction_settings: AuctionSettings,
    vesting_schedule: VestingSchedule
) -> Result<()> {
    ctx.accounts.auction_info.min_price = auction_settings.min_price;
    ctx.accounts.auction_info.max_price = auction_settings.max_price;
    ctx.accounts.auction_info.auction_start_time = auction_settings.start_time;
    ctx.accounts.auction_info.auction_end_time = auction_settings.end_time;

    ctx.accounts.auction_info.release_interval = vesting_schedule.release_interval;
    ctx.accounts.auction_info.release_rate = vesting_schedule.release_rate;
    ctx.accounts.auction_info.instant_unlock = vesting_schedule.instant_unlock;
    ctx.accounts.auction_info.initial_unlock = vesting_schedule.initial_unlock;
    ctx.accounts.auction_info.lock_period = vesting_schedule.lock_period;
    ctx.accounts.auction_info.vesting_period = vesting_schedule.vesting_period;
    ctx.accounts.auction_info.vesting_start_time = vesting_schedule.start_time;

    Ok(())
}
