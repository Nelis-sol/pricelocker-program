use crate::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};


// Unlock funds when the strike_time (input earlier by the user) is hit
#[derive(Accounts)]
#[instruction(locker_name: String)]
pub struct TimeUnlockFunds<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub authority_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"locker".as_ref(), authority.key().as_ref(), &locker_name.as_ref()],
        has_one = authority, 
        bump)]
    pub locker_pda: Account<'info, Locker>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = locker_pda,
    )]
    pub locker_token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> TimeUnlockFunds<'_> {
    pub fn process(&mut self, lock_index: u8) -> Result<()> {
        let Self {ref mut locker_pda,..} = self;

        // Users can choose whether they want to unlock all unlockable funds,or just a specific one
        // Lock_index is the index used to retrieve the lock object in the locks vector
        match lock_index {
            // index 255 is code for: unlock all unlockable-locks
            255 => {

                // Loops through all available locks
                for lock_item in &mut locker_pda.locks {

                    let clock: Clock = Clock::get().unwrap();
                    let time_now = clock.unix_timestamp as u32;

                    // Check if price lock can be openend (asset price exceeds strike price)
                    // Unlock locker if true
                    process_time_lock(lock_item, time_now).unwrap();
                }
            },

            // any other index than 255 leads to trying to retrieve the lock and unlock the funds
            index => {

                let clock: Clock = Clock::get().unwrap();
                let time_now = clock.unix_timestamp as u32;

                // Retrieves price lock from locks vector by the index
                let lock_item = locker_pda.locks
                    .get_mut(index as usize)
                    .ok_or(LockerErrorCode::NoLockAtIndex)
                    .unwrap();

                // Check if price lock can be openend (asset price exceeds strike price)
                // Unlock locker if true
                process_time_lock(lock_item, time_now).unwrap();

            }
        }

        Ok(())

    }
}            


// Open up locks of which the current price is larger than the strike_price stated in the lock (as earlier defined by the user)
fn process_time_lock<'info>(lock_item: &mut Lock, time_now: u32) -> Result<()> {

    // Check if lock is a price lock, and if so access the values 
    if let Lock::TimeLock { strike_time, locked, .. } = lock_item {
        // Check if the current time exceeds the strike_time defined in the locker
        if time_now >= *strike_time {

            // Unlock the lock
            *locked = false;

            Ok(())
            
        } else {
            Err(LockerErrorCode::TimeLowerThanStrikeTime.into())
        }
    } else {
        Err(LockerErrorCode::NotATimeLock.into())
    }
}

