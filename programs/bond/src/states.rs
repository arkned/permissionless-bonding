use anchor_lang::prelude::*;

#[derive(Default, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct VestingSchedule {
    /// Intervals that the release happens. Every interval, releaseRate of tokens are released.
    pub release_interval: u64,
    /// Release percent in each withdrawing interval
    pub release_rate: u64,
    /// Percent of tokens unlocked instantly before lock period
    pub instant_unlock: u64,
    /// Percent of tokens initially unlocked
    pub initial_unlock: u64,
    /// Period before release vesting starts, also it unlocks initialUnlock reward tokens. (in time unit of block.timestamp)
    pub lock_period: u64,
    /// Period to release all reward token, after lockPeriod + vestingPeriod it releases 100% of reward tokens. (in time unit of block.timestamp)
    pub vesting_period: u64,
}


#[derive(Default, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct DiscountSettings {
    /// minimum discount in bips.
    pub min_discout: u64,
    /// maximum discount in bips
    pub max_discount: u64,
    /// 1 -> discount rate grows from min to max, 2 -> max to min
    pub discount_mode: u64,
}

#[account]
#[derive(Default)]
pub struct ProjectInfo {
    /// Owner address(project)
    pub project_owner: Pubkey,

    /*************************** Project Info *************************/

    /// Project token
    pub project_token: Pubkey,
    /// LP token(from the project)
    pub lp_token: Pubkey,
    /// Account to receive bonded lps
    pub lp_token_account: Pubkey,
    /// Token amount
    pub token_amount: u64,
    /// Price
    pub price: u64,

    /*************************** Discount *************************/

    /// minimum discount in bips.
    pub min_discout: u64,
    /// maximum discount in bips
    pub max_discount: u64,
    /// 1 -> discount rate grows from min to max, 2 -> max to min
    pub discount_mode: u64,

    /*************************** Vesting Schedule *************************/

    /// Intervals that the release happens. Every interval, releaseRate of tokens are released.
    pub release_interval: u64,
    /// Release percent in each withdrawing interval
    pub release_rate: u64,
    /// Percent of tokens unlocked instantly before lock period
    pub instant_unlock: u64,
    /// Percent of tokens initially unlocked
    pub initial_unlock: u64,
    /// Period before release vesting starts, also it unlocks initialUnlock reward tokens. (in time unit of block.timestamp)
    pub lock_period: u64,
    /// Period to release all reward token, after lockPeriod + vestingPeriod it releases 100% of reward tokens. (in time unit of block.timestamp)
    pub vesting_period: u64,

    /*************************** Bonding Status *************************/

    /// Bonded lp token amount
    pub bonded_lp_amount: u64,
    /// Vested amount
    pub vested_amount: u64,
}


#[account]
#[derive(Default)]
pub struct BondsInfo {
    pub total_bonds: u64
}


#[account]
#[derive(Default)]
pub struct VestingInfo {
    /// Total amount of tokens to be vested.
    pub total_amount: u64,
    /// The amount that has been withdrawn.
    pub withdrawn_amount: u64,
    /// Start time of vesting
    pub start_time: u64,
}
