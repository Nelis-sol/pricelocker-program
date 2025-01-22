use crate::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};


// Add a price lock to the locker
// This will ensure the user can not access the funds until the price of the token is above the set strike price
#[derive(Accounts)]
#[instruction(locker_name: String, price_feed: Pubkey)]
pub struct PriceLockFunds<'info> {
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

impl<'info> PriceLockFunds<'_> {
    pub fn process(&mut self, price_feed: Pubkey, strike_price: u32, amount: u32, join: Option<u8>) -> Result<()> {
        let Self {locker_pda, locker_token_account, ..} = self;

        // Check if the amount to lock up is more than 0, otherwise the lock is not locking any funds
        require!((amount > 0), LockerErrorCode::PayoutAmountNotPositive);

        // Get total balance of the locker
        let mut available_balance = locker_token_account.amount;

        // Iterate through locks to subtract the locked amounts from the available balance
        // When user has 0 locks yet, the available balance will equal the total balance
        // With every lock, funds are locked and not available for a new lock
        for lock_item in &locker_pda.locks {  
            // Do checks and retrieve amount of funds locked
            let lock_item_balance = get_price_locked_balance(lock_item);
            // Subtract the funds locked from the available balance
            available_balance -= lock_item_balance;
        }

        // Check if the amount the user wants to lock is equal or lower than the available balance
        require!(((amount as u64) <= available_balance), LockerErrorCode::PayoutAmountExceedsAvailableBalance);

        // Get the next lock id (.len() is the index of the next lock, as index 0 is the first lock)
        let lock_id: u8 = locker_pda.locks.len() as u8;

        // Construct the new price lock object 
        let new_price_lock = Lock::PriceLock{
            id: lock_id,
            strike_price: strike_price,
            amount: amount,
            // For now we only support $SOL
            // When SPL are supported, the token_mint will cary the mint of the SPL token
            price_feed: price_feed,
            locked: true,
            join: join,
        };

        // Add price lock to the locker vector in the locker pda
        locker_pda.locks.push(new_price_lock);
        
        // Update the locked and unlocked balance
        locker_pda.locked_balance += amount;

        Ok(())
    }
}


// Perform checks and get the locked balance from a lock
fn get_price_locked_balance<'info>(lock_item: &Lock) -> u64 {

    // Check if lock is a price lock, and if so access the values 
    if let Lock::PriceLock { amount, locked, .. } = lock_item {
        
        // check if it is locked
        if *locked == true {
                // Lock is locked, retrieve the locked balance
                let locked_balance = *amount as u64;
                return locked_balance

        } else {
            // the price lock is unlocked, so the locked amount is 0
            return 0;
        }
    } else {
        // Lock is not a Price lock
        return 0;
    }
}