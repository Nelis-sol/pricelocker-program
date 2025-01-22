use crate::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};


// Lockers are differentiated by a locker_name, so 1 user can have multiple lockers
// Each locker accomodates 1 token account (one mint). For multiple mints, multiple lockers are required
// One locker can have multiple locks (e.g. 1 time lock and 1 price lock, max 10 locks), 
// There can be multiple conditions (AND/OR) for unlocking user funds
#[derive(Accounts)]
#[instruction(locker_name: String)]
pub struct CreateNewLocker<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init, 
        payer = authority,
        // Initialize with fixed space
        space = 8 + mem::size_of::<Locker>() + 100, 
        seeds = [b"locker".as_ref(), authority.key().as_ref(), locker_name.as_ref()], 
        bump)]
    pub locker_pda: Account<'info, Locker>,
    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        // Locker_pda is the authority of the token account
        associated_token::authority = locker_pda,
    )]
    pub locker_token_account: InterfaceAccount<'info, TokenAccount>,
    // Interface accounts are used to enable both token and token22 mints
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> CreateNewLocker<'_> {
    pub fn process(&mut self, locker_name: String, bump: u8) -> Result<()> {

        let Self {authority, locker_pda, mint,..} = self;

        // set signer as authority for the locker pda
        locker_pda.authority = authority.key();
        locker_pda.name = locker_name.clone();
        locker_pda.token_mint = mint.key();

        let clock: Clock = Clock::get().unwrap();
        locker_pda.creation_ts = clock.unix_timestamp as u32;

        // unlocked_balance starts at 0 as there is no funds deposited yet
        // locker.unlocked_balance = 0;
        locker_pda.locked_balance = 0;

        locker_pda.locked = false;
        
        locker_pda.bump = bump;

       Ok(())

    }
}