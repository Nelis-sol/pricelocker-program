use crate::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};


// Withdraw funds from the locker
// Only tokens that are not locked by a price or time lock can be withdrawn
#[derive(Accounts)]
#[instruction(locker_name: String)]
pub struct WithdrawUnlockedFunds<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
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
        mut,
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

impl<'info> WithdrawUnlockedFunds<'_> {
    pub fn process(&mut self, amount: u64, locker_name: String) -> Result<()> {
        let Self { authority, locker_pda, locker_token_account, token_program, authority_token_account, .. } = self;

        // Check if the authority is the same as the authority of the locker
        // Already checked in the instruction using the Anchor macro, but we do it again here for safety
        assert!(authority.key() == locker_pda.authority, "Signer not authorized");

        // Check if the payout amount is more than 0, otherwise the lock is not locking any funds
        require!((amount > 0), LockerErrorCode::PayoutAmountNotPositive);

        // Get total balance of the locker
        let available_balance = locker_token_account.amount;

        // Initiate locked_balance which we will fill when we find locked locks
        let mut locked_balance = 0;

        // Joined locks is a HashSet to find locks that are joined together (join = id of another lock)
        let mut joined_locks = HashSet::new();

        // Iterator through locks to subtract the locked amounts from the available balance
        // When user has 0 locks yet, the available balance will equal the total balance
        // With every lock, funds are locked and not available for a new lock which will be reflected in the locked_balance variable
        for lock_item in &locker_pda.locks { 

            match lock_item {
                // Access values of Lock items
                Lock::PriceLock { id, amount, locked, join, .. } | Lock::TimeLock { id, amount, locked, join, .. } => {
                    // Check if the lock is locked and has no dependency on other locks
                    if *locked == true && join.is_none() {
                        locked_balance += amount;
                    // Lock is locked and has a join with another lock, e.g. 100 $SOL has two locks: time lock (01-01-2025) and a price lock ($1000)
                    // We need to check if both locks are locked or if one is unlocked, in that case the funds are unlocked
                    // A user in this example basically says: unlock after 01-01-2025 OR if the $SOL price hits $1000
                    } else if *locked == true {
                        // Check if the id is already in the joined_locks
                        if !joined_locks.contains(&id) {
                            // Add the id to joined_locks
                            joined_locks.insert(id);
                        } else {
                            // The lock id is already in joined_locks
                            // This means both the locks are locked and the locked amount should be added to the locked_balance
                            locked_balance += amount;
                        }
                    }
                }
            } 
        }

        // Cast from u32 to u64
        let balance_locked = locked_balance as u64;

        // Subtract the locked amounts from the available balance
        let transfer_amount = available_balance - balance_locked;

        // Check if the amount the user wants withdraw is within the available balance
        require!((transfer_amount <= available_balance), LockerErrorCode::PayoutAmountExceedsAvailableBalance);

        let authority_key = authority.key();

        // Get the seeds for the authority (the locker_pda) of the token account
        let seeds = &[
            b"locker".as_ref(),
            authority_key.as_ref(),
            &locker_name.as_ref(),
            &[locker_pda.bump],
        ];

        // Transfer tokens out of the locker to the authority/user
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: locker_token_account.to_account_info(),
                    to: authority_token_account.to_account_info(),
                    authority: locker_pda.to_account_info(),
                },
                &[&seeds[..]],
            ),
            amount,
        )?;

        
        Ok(())
    }
}  
