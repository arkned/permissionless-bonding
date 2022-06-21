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

    pub fn init_new_project(ctx: Context<InitNewProject>, amount: u64, price: u64, discount_settings: DiscountSettings, vesting_schedule: VestingSchedule) -> Result<()> {
        process_init_new_project(ctx, amount, price, discount_settings, vesting_schedule)
    }

    pub fn update_authority(ctx: Context<UpdateAuthority>, _project_bonding_id: u64, new_authority: Pubkey) -> Result<()> {
        process_update_authority(ctx, new_authority)
    }

    pub fn update_price(ctx: Context<UpdatePrice>, _project_bonding_id: u64, new_price: u64) -> Result<()> {
        process_update_price(ctx, new_price)
    }

    pub fn bond(ctx: Context<Bond>, _project_bonding_id: u64, lp_amount: u64) -> Result<()> {
        process_bond(ctx, lp_amount)
    }

    pub fn withdraw_vesting(ctx: Context<WithdrawVesting>, project_bonding_id: u64, _bond_id: u64) -> Result<()> {
        process_withdraw_vesting(ctx, project_bonding_id)
    }
}
