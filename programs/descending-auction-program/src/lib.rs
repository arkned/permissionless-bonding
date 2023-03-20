use std::convert::TryFrom;
use std::mem::size_of;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, Token, TokenAccount, Transfer};
use spl_math::{precise_number::PreciseNumber, uint::U256};

#[cfg(feature = "local-testing")]
declare_id!("CwuWwv57X9Yerfhkh9oEDJzr1qgyDFYr2mkyZ3HH8jjJ");

#[cfg(not(feature = "local-testing"))]
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub const AUCTION_PREFIX: &[u8] = b"auction";

#[program]
pub mod descending_auction_program {
    // DELETEME: std::convert::TryInto is now included in prelude in rust 2021 edition
    // Make sure we'll be using 2021 edition
    //use std::convert::TryInto;

    use super::*;

    //
    // Admin-facing instructions:
    //
    /// Sets up a new auction
    ///
    /// preconditions:
    ///  - `start_timestamp < end_timestamp`
    ///  - `floor_price < ceil_price`
    ///  - `floor_price > 0`
    #[allow(clippy::too_many_arguments)]
    pub fn initialize_auction(
        ctx: Context<InitializeAuction>,
        start_timestamp: i64,
        end_timestamp: i64, // TODO: should this be an option type to represent open-ended auction?
        ceil_price: u64,    // represented as # of payment tokens per purchase token
        floor_price: u64,
        price_hold_duration: i64, // in seconds  // TODO: delete this
        auction_authority_bump: u8,
        auction_pool_bump: u8,
    ) -> ProgramResult {
        // get given accounts
        let auction = &mut ctx.accounts.auction;
        let authority = &ctx.accounts.authority;
        let payment_mint = &ctx.accounts.payment_mint;
        let payment_destination = &ctx.accounts.payment_destination;
        let sale_mint = &ctx.accounts.sale_mint;
        let auction_pool = &ctx.accounts.auction_pool;

        // assert preconditions of the instruction
        if end_timestamp <= start_timestamp {
            msg!("Ending time of auction cannot be before starting time");
            return Err(AuctionError::InvalidAuctionTimestamps.into());
        }
        if ceil_price <= floor_price {
            msg!("Auction price has to be none zero");
            return Err(AuctionError::InvalidAuctionPrice.into());
        }

        // populate auction account
        auction.authority = authority.key();
        auction.start_timestamp = start_timestamp;
        auction.end_timestamp = end_timestamp;
        auction.payment_mint = payment_mint.key();
        auction.payment_destination = payment_destination.key();
        auction.sale_mint = sale_mint.key();
        auction.auction_pool = auction_pool.key();
        auction.ceil_price = ceil_price;
        auction.floor_price = floor_price;
        auction.price_hold_duration = price_hold_duration;
        auction.auction_authority_bump = auction_authority_bump;
        auction.auction_pool_bump = auction_pool_bump;
        auction.last_purchase_timestamp = None;
        auction.last_purchase_price = ceil_price;

        Ok(())
    }

    /// Updates the end time of an auction
    ///
    /// preconditions:
    ///  - `auction.start_timestamp < end_timestamp`
    ///  - auction is pending
    #[access_control(ctx.accounts.auction.assert_auction_state(AuctionState::Pending))]
    pub fn update_end_time(ctx: Context<UpdateEndTime>, end_timestamp: i64) -> ProgramResult {
        let auction = &mut ctx.accounts.auction;
        auction.end_timestamp = end_timestamp;

        Ok(())
    }

    /// Updates the start time of an auction
    ///
    /// preconditions:
    ///  - `start_timestamp < auction.end_timestamp`
    ///  - `current_timestamp < start_timestamp`
    ///  - auction is pending
    #[access_control(ctx.accounts.auction.assert_auction_state(AuctionState::Pending))]
    pub fn update_start_time(ctx: Context<UpdateStartTime>, start_timestamp: i64) -> ProgramResult {
        let current_timestamp = Clock::get()?.unix_timestamp;

        if start_timestamp <= current_timestamp {
            msg!("Starting time of auction cannot be before current time");
            return Err(AuctionError::InvalidAuctionTimestamps.into());
        }

        let auction = &mut ctx.accounts.auction;
        auction.start_timestamp = start_timestamp;

        Ok(())
    }

    /// Updates the ceiling price of an auction
    ///
    /// preconditions:
    ///  - `auction.floor_price < ceil_price`
    ///  - auction is pending
    #[access_control(ctx.accounts.auction.assert_auction_state(AuctionState::Pending))]
    pub fn update_ceil_price(ctx: Context<UpdateCeilPrice>, ceil_price: u64) -> ProgramResult {
        let auction = &mut ctx.accounts.auction;
        auction.ceil_price = ceil_price;

        Ok(())
    }

    /// Updates the floor price of an auction
    ///
    /// preconditions:
    ///  - `floor_price < auction.ceil_price`
    ///  - `floor_price > 0`
    ///  - auction is pending
    #[access_control(ctx.accounts.auction.assert_auction_state(AuctionState::Pending))]
    pub fn update_floor_price(ctx: Context<UpdateFloorPrice>, floor_price: u64) -> ProgramResult {
        let auction = &mut ctx.accounts.auction;
        auction.floor_price = floor_price;

        Ok(())
    }

    /// Deposits a given amount of sale tokens into an auction pool
    ///
    /// preconditions:
    ///  - auction is pending
    #[access_control(ctx.accounts.auction.assert_auction_state(AuctionState::Pending))]
    pub fn deposit_to_auction_pool(
        ctx: Context<DepositToAuctionPool>,
        deposit_amount: u64,
    ) -> ProgramResult {
        // get given accounts
        let authority = &ctx.accounts.authority;
        let source_account = &ctx.accounts.source_account;
        let auction_pool = &ctx.accounts.auction_pool;
        let token_program = &ctx.accounts.token_program;

        // CPI to token transfer instruction
        let deposit_accounts = Transfer {
            from: source_account.to_account_info(),
            to: auction_pool.to_account_info(),
            authority: authority.to_account_info(),
        };
        let deposit_ctx = CpiContext::new(token_program.to_account_info(), deposit_accounts);
        token::transfer(deposit_ctx, deposit_amount)?;

        Ok(())
    }

    /// Close a pending or ended auction and refund the sale token back
    ///
    /// preconditions:
    ///  - auction is pending or ended
    #[access_control(ctx.accounts.auction.assert_auction_state_not_in_progress())]
    pub fn close_auction(ctx: Context<CloseAuction>) -> ProgramResult {
        let auction = &ctx.accounts.auction;
        let auction_authority = &ctx.accounts.auction_authority;
        let auction_pool = &ctx.accounts.auction_pool;
        let authority = &ctx.accounts.authority;
        let destination_account = &ctx.accounts.destination_account;
        let token_program = &ctx.accounts.token_program;

        let auction_authority_seeds = &[
            AUCTION_PREFIX,
            &auction.key().to_bytes(),
            &[auction.auction_authority_bump],
        ];
        let auction_authority_signer = &[&auction_authority_seeds[..]];

        // refund sale token from auction pool
        if auction_pool.amount > 0 {
            msg!("Transferring remaining sale tokens");
            let refund_accounts = Transfer {
                from: auction_pool.to_account_info(),
                to: destination_account.to_account_info(),
                authority: auction_authority.to_account_info(),
            };
            let refund_ctx = CpiContext::new_with_signer(
                token_program.to_account_info(),
                refund_accounts,
                auction_authority_signer,
            );
            token::transfer(refund_ctx, auction_pool.amount)?;
        }

        // close auction pool token account and refund lamports
        let close_accounts = CloseAccount {
            account: auction_pool.to_account_info(),
            destination: authority.to_account_info(),
            authority: auction_authority.to_account_info(),
        };
        let close_ctx = CpiContext::new_with_signer(
            token_program.to_account_info(),
            close_accounts,
            auction_authority_signer,
        );
        token::close_account(close_ctx)?;

        Ok(())
    }

    //
    // User-facing instructions:
    //
    /// Purchase token from an auction given a purchasing amount
    ///
    /// preconditions:
    ///  - auction is in progress
    // TODO:
    //  - accept expected price and slippage tolerance
    //  - if the slippage is higher than the given tolerance, cancel the tx with an error
    //  - document how slippage tolerance is expressed (permille as integer)
    //  - document how purchase_amount is expressed (sale token)
    #[access_control(ctx.accounts.auction.assert_auction_state(AuctionState::InProgress))]
    pub fn purchase(
        ctx: Context<Purchase>,
        purchase_amount: u64, // in sale token (whole number mutiple of the smallest unit)
        expected_payment: u64, // in payment token
        slippage_tolerance: u64, // permille as unsigned integer (min: 0; max:1000)
    ) -> ProgramResult {
        let current_timestamp = Clock::get()?.unix_timestamp;

        // get given accounts
        let auction = &mut ctx.accounts.auction;
        let auction_authority = &ctx.accounts.auction_authority;
        let sale_mint = &ctx.accounts.sale_mint;
        let auction_pool = &ctx.accounts.auction_pool;
        let payment_destination = &ctx.accounts.payment_destination;
        let buyer = &ctx.accounts.buyer;
        let payment_source = &ctx.accounts.payment_source;
        let sale_destination = &ctx.accounts.sale_destination;

        let token_program = &ctx.accounts.token_program;

        // TODO: decide which token is going to be the base token of the trade; we can:
        //  -> calculate the amount of payment tokens to accept based on the amount of sale tokens
        //  -  calculate the amount of sale tokens to dispense based on the amount of payment tokens
        msg!("calculate auction invoice");
        // NOTE:
        //  - `purchase_amount` is the amount of unit sale token for purchase
        //  - `payment_amount` is the (least) amount of unit payment token suffices to be exchanged to purchase_amount
        let payment_amount = auction.get_payment_amount(purchase_amount, sale_mint.decimals)?;

        // TODO: validation for the trade:
        // 1. make sure the price is non-zero
        // 2. TODO: do i need to double check the pool balance?

        // check if the calculated price exceeds the given slippage
        if payment_amount > expected_payment {
            let slippage_expected_payment = (expected_payment as u128)
                .checked_mul((slippage_tolerance as u128) + 1_000)
                .and_then(|v| v.checked_div(1_000))
                .ok_or(AuctionError::InternalError)?;

            if (payment_amount as u128) > slippage_expected_payment {
                return Err(AuctionError::PurchasePriceOutOfSlippage.into());
            }
        }

        // accept the payment
        msg!("Transferring payment token");
        let payment_accounts = Transfer {
            from: payment_source.to_account_info(),
            to: payment_destination.to_account_info(),
            authority: buyer.to_account_info(),
        };
        let payment_ctx = CpiContext::new(token_program.to_account_info(), payment_accounts);
        token::transfer(payment_ctx, payment_amount)?;

        // dispense the sale token to the buyer
        msg!("Transferring sale token");
        let sale_accounts = Transfer {
            from: auction_pool.to_account_info(),
            to: sale_destination.to_account_info(),
            authority: auction_authority.to_account_info(),
        };
        let auction_authority_seeds = &[
            AUCTION_PREFIX,
            &auction.key().to_bytes(),
            &[auction.auction_authority_bump],
        ];
        let auction_authority_signer = &[&auction_authority_seeds[..]];
        let sale_ctx = CpiContext::new_with_signer(
            token_program.to_account_info(),
            sale_accounts,
            auction_authority_signer,
        );
        token::transfer(sale_ctx, purchase_amount)?;

        // update the on-chain state for this purchase
        msg!("Update timestamp for last purchase");
        auction.last_purchase_timestamp = Some(current_timestamp);

        Ok(())
    }
}

//
// instructions.rs
//
#[derive(Accounts)]
#[instruction(
    _start_timestamp: i64,
    _end_timestamp: i64,
    _ceil_price: u64,
    _floor_price: u64,
    _price_hold_duration: i64,
    auction_authority_bump: u8,
    auction_pool_bump: u8,
)]
pub struct InitializeAuction<'info> {
    /// The `Auction` account to be initialize
    #[account(
        init,
        payer = authority,
        space = Auction::LEN,
    )]
    pub auction: Account<'info, Auction>,

    /// (PDA) The authority assigned to each auction that controls auction pool
    #[account(
        seeds = [AUCTION_PREFIX, &auction.key().to_bytes()],
        bump = auction_authority_bump,
    )]
    pub auction_authority: AccountInfo<'info>,

    /// The authority that controls the auction being initialized
    #[account(
        mut @ AuctionError::AuthorityNotMutable,
        signer @ AuctionError::AuthorityNotSigner,
    )]
    pub authority: AccountInfo<'info>,

    /// The mint that represents the type of token accepted as payment (payment token)
    pub payment_mint: Account<'info, Mint>,

    /// The payment token account that each payment will be sent to
    #[account(
        constraint = payment_destination.mint == payment_mint.key() @ AuctionError::InvalidPaymentDestination,
    )]
    pub payment_destination: Account<'info, TokenAccount>,

    /// The mint that represents the type of token being sold in the auction (sale token)
    pub sale_mint: Account<'info, Mint>,

    /// (PDA) The pool to hold sale tokens; controled by the auction authority
    #[account(
        init,
        payer = authority,
        token::mint = sale_mint,
        token::authority = auction_authority,
        seeds = [AUCTION_PREFIX, &auction.key().to_bytes(), &sale_mint.key().to_bytes()],
        bump = auction_pool_bump,
    )]
    pub auction_pool: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(end_timestamp: i64)]
pub struct UpdateEndTime<'info> {
    /// The auction to modify
    #[account(
        mut,
        has_one = authority @ AuctionError::InvalidAuctionAuthority,
        constraint = auction.start_timestamp < end_timestamp  @ AuctionError::InvalidAuctionTimestamps,
    )]
    pub auction: Account<'info, Auction>,

    /// The authority that controls the provided auction
    #[account(
        signer @ AuctionError::AuthorityNotSigner
    )]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(start_timestamp: i64)]
pub struct UpdateStartTime<'info> {
    /// The auction to modify
    #[account(
        mut,
        has_one = authority @ AuctionError::InvalidAuctionAuthority,
        constraint = auction.end_timestamp > start_timestamp  @ AuctionError::InvalidAuctionTimestamps,
    )]
    pub auction: Account<'info, Auction>,

    /// The authority that controls the provided auction
    #[account(
        signer @ AuctionError::AuthorityNotSigner,
    )]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(ceil_price: u64)]
pub struct UpdateCeilPrice<'info> {
    /// The auction to modify
    #[account(
        mut,
        has_one = authority @ AuctionError::InvalidAuctionAuthority,
        constraint = auction.floor_price < ceil_price @ AuctionError::InvalidAuctionPrice,
    )]
    pub auction: Account<'info, Auction>,

    /// The authority that controls the provided auction
    #[account(
        signer @ AuctionError::AuthorityNotSigner,
    )]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(floor_price: u64)]
pub struct UpdateFloorPrice<'info> {
    /// The auction to modify
    #[account(
        mut,
        has_one = authority @ AuctionError::InvalidAuctionAuthority,
        constraint = 0 < floor_price && floor_price < auction.ceil_price @ AuctionError::InvalidAuctionPrice,
    )]
    pub auction: Account<'info, Auction>,

    /// The authority that controls the provided auction
    #[account(
        signer @ AuctionError::AuthorityNotSigner,
    )]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(deposit_amount: u64)]
pub struct DepositToAuctionPool<'info> {
    /// The auction to deposit the sale tokens to
    // NOTE: if we want to allow depositing to an `InProgress` auction then the constraint has to change
    #[account(
        has_one = authority @ AuctionError::InvalidAuctionAuthority,
    )]
    pub auction: Account<'info, Auction>,

    /// (PDA) The pool to deposit sale tokens; controled by the auction authority
    #[account(
        mut,
        seeds = [AUCTION_PREFIX, &auction.key().to_bytes(), &auction.sale_mint.to_bytes()],
        bump = auction.auction_pool_bump,
        constraint = auction_pool.owner == auction_authority.key() @ AuctionError::AuctionPoolNotOwnedByAuction,
    )]
    pub auction_pool: Account<'info, TokenAccount>,

    /// (PDA) The authority assigned to the auction
    #[account(
        seeds = [AUCTION_PREFIX, &auction.key().to_bytes()],
        bump = auction.auction_authority_bump,
    )]
    pub auction_authority: AccountInfo<'info>,

    /// The authority that controls the provided auction
    #[account(
        signer @ AuctionError::AuthorityNotSigner,
    )]
    pub authority: AccountInfo<'info>,

    /// The sale token account that will fund the auction pool
    #[account(mut)]
    pub source_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CloseAuction<'info> {
    /// The auction to be closed
    #[account(
        mut,
        has_one = authority @ AuctionError::InvalidAuctionAuthority,
        close = authority,
    )]
    pub auction: Account<'info, Auction>,

    /// (PDA) The pool that holds sale tokens; controled by the auction authority
    #[account(
        mut,
        seeds = [AUCTION_PREFIX, &auction.key().to_bytes(), &auction.sale_mint.to_bytes()],
        bump = auction.auction_pool_bump,
        constraint = auction_pool.owner == auction_authority.key() @ AuctionError::AuctionPoolNotOwnedByAuction,
    )]
    pub auction_pool: Account<'info, TokenAccount>,

    /// (PDA) The authority assigned to the auction
    #[account(
        seeds = [AUCTION_PREFIX, &auction.key().to_bytes()],
        bump = auction.auction_authority_bump,
    )]
    pub auction_authority: AccountInfo<'info>,

    /// The authority that controls the provided auction
    #[account(
        signer @ AuctionError::AuthorityNotSigner,
    )]
    pub authority: AccountInfo<'info>,

    /// The sale token account to refund to
    #[account(mut)]
    pub destination_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(purchase_amount: u64)]
pub struct Purchase<'info> {
    /// The auction to purchase from
    #[account(mut)]
    pub auction: Account<'info, Auction>,

    /// (PDA) The auction authority to sign the sale token transfer
    #[account(
        seeds = [AUCTION_PREFIX, &auction.key().to_bytes()],
        bump = auction.auction_authority_bump,
    )]
    pub auction_authority: AccountInfo<'info>,

    /// The sale token mint
    #[account(
        constraint = sale_mint.key() == auction.sale_mint,
    )]
    pub sale_mint: Box<Account<'info, Mint>>,

    /// (PDA) The auction pool to withdraw sale tokens from
    #[account(
        mut,
        seeds = [AUCTION_PREFIX, &auction.key().to_bytes(), &auction.sale_mint.to_bytes()],
        bump = auction.auction_pool_bump,
        constraint = auction_pool.owner == auction_authority.key() @ AuctionError::AuctionPoolNotOwnedByAuction,
        constraint = auction_pool.mint == auction.sale_mint @ AuctionError::AuctionPoolWrongMint,
        constraint = auction_pool.amount >= purchase_amount @ AuctionError::AuctionPoolBalanceTooLow,
    )]
    pub auction_pool: Box<Account<'info, TokenAccount>>,

    /// The sale token account that the payment will be sent to
    #[account(
        mut,
        constraint = payment_destination.mint == auction.payment_mint @ AuctionError::InvalidPaymentDestination,
        constraint = payment_destination.key() == auction.payment_destination @ AuctionError::InvalidPaymentDestination,
    )]
    pub payment_destination: Account<'info, TokenAccount>,

    /// The buyer who signs the payment token transaction
    #[account(signer)]
    pub buyer: AccountInfo<'info>,

    /// The buyer's payment token account to fund the purchase
    #[account(
        mut,
        constraint = payment_source.owner == buyer.key() || payment_source.delegate.contains(&buyer.key()) @ AuctionError::PaymentSourceNotOwned,
        constraint = payment_source.mint == auction.payment_mint @ AuctionError::InvalidPaymentSource,
    )]
    pub payment_source: Account<'info, TokenAccount>,

    /// The buyer's sale token account to receive the sale token
    #[account(
        mut,
        constraint = sale_destination.mint == auction.sale_mint @ AuctionError::InvalidSaleDestinationMint,
    )]
    pub sale_destination: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

//
// errors.rs
//
#[error]
pub enum AuctionError {
    #[msg("The provided authority is not mutable")]
    AuthorityNotMutable, // 0x1770

    #[msg("The provided authority is not a signer")]
    AuthorityNotSigner, // 0x1771

    #[msg("The mint of the provided payment destination does not match the payment mint of the provided auction")]
    InvalidPaymentDestination, // 0x1772

    #[msg("The mint of the provided payment source does not match the payment mint of the provided auction")]
    InvalidPaymentSource, // 0x1773

    #[msg("The provided timestamps for auction do not describe a valid length of time")]
    InvalidAuctionTimestamps, // 0x1774

    #[msg("The provided authority does not have authority over the provided auction")]
    InvalidAuctionAuthority, // 0x1775

    #[msg("The provided auction must be in the Pending state")]
    AuctionNotPending, // 0x1776

    #[msg("The provided auction must be in the InProgress state")]
    AuctionNotInProgress, // 0x1777

    #[msg("The provided auction must be in the Ended state")]
    AuctionNotEnded, // 0x1778

    #[msg("The provided auction must be in either the Pending or Ended state")]
    AuctionInProgress, // 0x1779

    #[msg("The provided auction pool is not owned by the provided auction")]
    AuctionPoolNotOwnedByAuction, // 0x177a

    #[msg("The provided payment source is not owned by the buyer")]
    PaymentSourceNotOwned, // 0x177b

    #[msg("The provided auction price is not valid")]
    InvalidAuctionPrice, // 0x177c

    #[msg("The mint of the provided auction pool does not match the sale mint of the provided auction")]
    AuctionPoolWrongMint, // 0x177d

    #[msg("The provided purchase amount exceeds the balance of the auction pool")]
    AuctionPoolBalanceTooLow, // 0x177e

    #[msg("The mint of the provided sale destination does not match the sale mint of the provided auction")]
    InvalidSaleDestinationMint, // 0x177f

    #[msg("The price of the purchase instruction exceeded the provided slippage limit")]
    PurchasePriceOutOfSlippage, // 0x1780

    #[msg("Internal Error")]
    InternalError, // 0x1781
}

impl From<AuctionState> for AuctionError {
    fn from(state: AuctionState) -> Self {
        match state {
            AuctionState::Pending => Self::AuctionNotPending,
            AuctionState::InProgress => Self::AuctionNotInProgress,
            AuctionState::Ended => Self::AuctionNotEnded,
        }
    }
}

//
// state.rs
//
pub trait AnchorLen {
    const LEN: usize;
}

impl<T: AccountSerialize + AccountSerialize> AnchorLen for T {
    const LEN: usize = 8 + size_of::<Self>();
}

/// States of auction
#[derive(PartialEq)]
pub enum AuctionState {
    Pending,
    InProgress,
    Ended,
}

// TODO: How should the current price react when a purchase has been made?
//  - we could make it accept a function and used it to derive price given time passed from the last purchase?
//  - but for now, *keep it simple*:
//    - use `price_hold_duration` to have the price be held at the same value for a duration.
//    - initialize `last_purchase_timestamp` as the starting time of the auction and save the timestamp of the last purchase
//    - use it with `price_hold_duration` to hold the price for the specified duration
// TODO: decide how to represent the early cancelation of auction
//
// NOTE:
//  - client figures out the current price by looking at on-chain state
//  - client submits the bid through purchase instruction
//  - purchase instruction decides to approve or reject the submitted bid
//  - purchase instruction updates the on-chain state if approved
/// Represents an instance of an auction
///
/// Invariants:
///  - auction_pool.owner = auction_authority.key()
///  - `start_timestamp < end_timestamp`
///  -
#[account]
pub struct Auction {
    pub authority: Pubkey,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub payment_mint: Pubkey,
    pub payment_destination: Pubkey,
    pub sale_mint: Pubkey,
    pub auction_pool: Pubkey,
    pub ceil_price: u64,
    pub floor_price: u64,
    pub price_hold_duration: i64,
    pub auction_authority_bump: u8,
    pub auction_pool_bump: u8,
    // TODO: decide how the current price will be calculated with the information saved in auction account
    pub last_purchase_timestamp: Option<i64>,
    pub last_purchase_price: u64,
}

// TODO: define procedures to derive information from the on-chain state
// These are some of the questions that needs to be answered looking at the on-chain state
//  - is the auction started? can I buy it yet or what? or is it ended alread?
//  - what is the current price?
//  - how much has been sold?
impl Auction {
    pub fn get_current_auction_state(&self, current_timestamp: i64) -> AuctionState {
        if current_timestamp < self.start_timestamp {
            AuctionState::Pending
        } else if self.start_timestamp <= current_timestamp
            && current_timestamp < self.end_timestamp
        {
            AuctionState::InProgress
        } else {
            AuctionState::Ended
        }
    }

    pub fn assert_auction_state(&self, expected_state: AuctionState) -> ProgramResult {
        let clock = Clock::get()?;
        if self.get_current_auction_state(clock.unix_timestamp) != expected_state {
            Err(AuctionError::from(expected_state).into())
        } else {
            Ok(())
        }
    }

    /// Assert the auction to be not InProgress
    pub fn assert_auction_state_not_in_progress(&self) -> ProgramResult {
        self.assert_auction_state(AuctionState::Pending)
            .or(self.assert_auction_state(AuctionState::Ended))
            .map_err(|_| AuctionError::AuctionInProgress.into())
    }

    fn _get_duration(&self) -> Option<i64> {
        self.end_timestamp.checked_sub(self.start_timestamp)
    }

    /// Calculate the amount of payment token for a given amount of sale token at current time
    pub fn get_payment_amount(&self, purchase_amount: u64, sale_decimals: u8) -> Result<u64> {
        let base = PreciseNumber {
            value: U256::from(1_000_050_000_000u128),
        };

        // calculate the time since start of auction
        let t = (Clock::get()?.unix_timestamp - self.start_timestamp) as u128;
        // TODO: t -> f(t, remaining precentage of sale token)
        let decay_factor = base.checked_pow(t);

        // max(floor_price, current_price)
        let floor_price = PreciseNumber::new(self.floor_price as u128);
        let current_price = PreciseNumber::new(self.ceil_price as u128)
            .and_then(|v| v.checked_div(&decay_factor?))
            .and_then(|v| {
                let floor_price = floor_price?;
                Some(if v.greater_than(&floor_price) {
                    v
                } else {
                    floor_price
                })
            });

        // calculate the payment amount: price * sale_amount
        let sale_decimals = 10u128
            .checked_pow(sale_decimals.into())
            .and_then(PreciseNumber::new);

        PreciseNumber::new(purchase_amount as u128)
            .and_then(|v| v.checked_mul(&current_price?))
            .and_then(|v| v.checked_div(&sale_decimals?))
            .and_then(|v| v.ceiling())
            .and_then(|v| u64::try_from(v.to_imprecise()?).ok())
            .ok_or(AuctionError::InternalError.into())
    }
}
