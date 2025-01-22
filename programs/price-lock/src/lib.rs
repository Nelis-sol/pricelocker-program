use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

use std::collections::HashSet;
use std::mem;

pub mod contexts;
pub mod states;

pub use contexts::*;
pub use states::*;

declare_id!("BPVWMMj1eEALBvsR3TQZC2Zb3vt8jQkvzBYeJsoaKum7");

#[program]
pub mod price_lock {

    use super::*;

    // Create new price locker
    pub fn create_new_locker(
        ctx: Context<CreateNewLocker>,
        locker_name: String,
    ) -> Result<()> {
        let bump = ctx.bumps.locker_pda;
        ctx.accounts.process(locker_name, bump)
    }

    // Deposit to price locker
    pub fn deposit_new_funds(
        ctx: Context<DepositNewFunds>,
        locker_name: String,
        amount: u32,
    ) -> Result<()> {
        let transfer_amount: u64 = amount as u64;
        ctx.accounts.process(transfer_amount)
    }

    // Add a price lock to the locker
    pub fn price_lock_funds(
        ctx: Context<PriceLockFunds>,
        locker_name: String,
        price_feed: Pubkey,
        strike_price: u32,
        payout_amount: u32,
        join: Option<u8>,
    ) -> Result<()> {
        ctx.accounts
            .process(price_feed, strike_price, payout_amount, join)
    }

    // Add a time lock to the locker
    pub fn time_lock_funds(
        ctx: Context<TimeLockFunds>,
        locker_name: String,
        strike_time: u32,
        payout_amount: u32,
        join: Option<u8>,
    ) -> Result<()> {
        ctx.accounts
            .process(strike_time, payout_amount, join)
    }

    // Unlock a time lock
    pub fn time_unlock_funds(
        ctx: Context<TimeUnlockFunds>, 
        locker_name: String,
        lock_index: u8) -> Result<()> {
        ctx.accounts.process(lock_index)
    }

    // Unlock a price lock
    pub fn price_unlock_funds(
        ctx: Context<PriceUnlockFunds>, 
        lock_index: u8,
        locker_name: String,
    ) -> Result<()> {
        ctx.accounts.process(lock_index)
    }

    // Withdraw funds from the locker
    pub fn withdraw_unlocked_funds(
        ctx: Context<WithdrawUnlockedFunds>,
        locker_name: String,
        amount: u32, 
    ) -> Result<()> {
        let transfer_amount: u64 = amount as u64;
        ctx.accounts.process(transfer_amount, locker_name)
    }
}
