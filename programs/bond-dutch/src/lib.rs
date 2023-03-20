use anchor_lang::prelude::*;

/// constant
pub mod constant;
/// context accounts
pub mod context_accounts;
/// processor
pub mod processor;
/// states
pub mod states;

use crate::{context_accounts::*, processor::*, states::*};

declare_id!("B3HQ6SpGbgBD9ANG9dsFLxa3HWWkzdvYeFyy9TKbqrpF");

#[program]
pub mod bond {
    use super::*;

    pub fn init_auction(ctx: Context<InitAuction>, amount: u64, auction_settings: AuctionSettings, vesting_schedule: VestingSchedule) -> Result<()> {
        process_init_auction(ctx, amount, auction_settings, vesting_schedule)
    }

    pub fn update_authority(ctx: Context<UpdateAuthority>, _auction_id: u64, new_authority: Pubkey) -> Result<()> {
        process_update_authority(ctx, new_authority)
    }

    pub fn update_settings(ctx: Context<UpdateSettings>, _auction_id: u64, auction_settings: AuctionSettings, vesting_schedule: VestingSchedule) -> Result<()> {
        process_update_settings(ctx, auction_settings, vesting_schedule)
    }

    pub fn bond(ctx: Context<Bond>, _auction_id: u64, lp_amount: u64) -> Result<()> {
        process_bond(ctx, lp_amount)
    }

    pub fn end_auction(ctx: Context<EndAuction>, _auction_id: u64) -> Result<()> {
        process_end_auction(ctx)
    }

    pub fn withdraw_vesting(ctx: Context<WithdrawVesting>, _auction_id: u64, _bond_id: u64) -> Result<()> {
        process_withdraw_vesting(ctx)
    }
}
