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
    /// start time of the vesting
    pub start_time: u64,
}


#[derive(Default, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct AuctionSettings {
    /// max price of token
    pub max_price: u64,
    /// min price of toke
    pub min_price: u64,
    /// start time of the auction
    pub start_time: u64,
    /// end time of the auction
    pub end_time: u64,
}


#[account]
#[derive(Default)]
pub struct AuctionInfo {
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

    /*************************** Auction *************************/

    /// minimum price.
    pub min_price: u64,
    /// maximum price
    pub max_price: u64,
    /// start time of auction
    pub auction_start_time: u64,
    /// end time of auction
    pub auction_end_time: u64,

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
    /// Start time of vesting
    pub vesting_start_time: u64,

    /*************************** Bonding Status *************************/

    /// Bonded lp token amount
    pub bonded_lp_amount: u64,
    /// Final price
    pub final_price: u64,
    /// Is success
    pub is_auction_success: bool
}

#[account]
#[derive(Default)]
pub struct ProjectAuctions {
    pub next_auction_id: u64
}

#[account]
#[derive(Default)]
pub struct VestingInfo {
    /// Total amount of bonded lp tokens.
    pub bonded_lp_amount: u64,
    /// The amount that has been withdrawn.
    pub withdrawn_amount: u64,
}
