use crate::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

// Unlock funds when the strike_price (input earlier by the user) is hit
// Uses the Pyth price oracle to determine current (SOL) price
#[derive(Accounts)]
#[instruction(index: u8, locker_name: String)]
pub struct PriceUnlockFunds<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub authority_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut,
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
    #[account(owner = pyth_solana_receiver_sdk::ID)]
    pub pyth_price_update_account: Account<'info, PriceUpdateV2>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> PriceUnlockFunds<'_> {
    pub fn process(&mut self, lock_index: u8) -> Result<()> {

        msg!("price_unlock_funds");

        let Self {ref mut locker_pda, pyth_price_update_account,..} = self;

        // Users can choose whether they want to unlock all unlockable funds,or just a specific one
        // Lock_index is the index used to retrieve the lock object in the locks vector
        match lock_index {
            // index 255 is code for: unlock all unlockable-locks
            255 => {

                // Loops through all available locks
                for lock_item in &mut locker_pda.locks {

                    // Check if price lock can be openend (asset price exceeds strike price)
                    // Unlock locker if true
                    process_price_lock(lock_item, pyth_price_update_account).unwrap();
                }
            },
            // any other index than 255 leads to trying to retrieve the lock and unlock the funds
            index => {

                // Retrieves price lock from locks vector by the index
                let lock_item = locker_pda.locks
                    .get_mut(index as usize)
                    .ok_or(LockerErrorCode::NoLockAtIndex)
                    .unwrap();

                // Check if price lock can be openend (asset price exceeds strike price)
                // Unlock locker if true
                process_price_lock(lock_item, pyth_price_update_account).unwrap();

            }
        }

        Ok(())

    }
}


// // Retrieve price from Pyth pricefeed for comparison with strike price
// fn get_price_from_pricefeed<'info>(pyth_price_update_account: &mut Account<'info, PriceUpdateV2>) -> Result<u32> {

//     // Get the current timestamp
//     let current_timestamp = Clock::get()?;
//     msg!("current_timestamp: {:?}", current_timestamp.unix_timestamp);

//     let feed_pk: [u8; 32] = Pubkey::from_str("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE").unwrap().to_bytes();

//     let sol_price_no_conf_interval = pyth_price_update_account.get_price_no_older_than(&current_timestamp, 600, &feed_pk)?;

//     msg!("pyth timestamp: {}", sol_price_no_conf_interval.publish_time);

//     let price = sol_price_no_conf_interval.price as u32;

//     Ok(price)
// }

// Open up locks of which the current price is larger than the strike_price stated in the lock (as earlier defined by the user)
fn process_price_lock<'info>(lock_item: &mut Lock, pyth_solprice_account: &mut Account<'info, PriceUpdateV2>) -> Result<()> {

    // Check if lock is a price lock, and if so access the values 
    if let Lock::PriceLock {  strike_price, locked, price_feed, .. } = lock_item {

        if *price_feed != pyth_solprice_account.key() {
            return Err(LockerErrorCode::PriceFeedNotMatchingPricefeedAccount.into());
        }

        msg!("pyth_price: {}", pyth_solprice_account.price_message.price);
        msg!("strike_price: {}", strike_price);

        let amount_in_lamports = LAMPORTS_PER_SOL
            .checked_mul(10_u64.pow(pyth_solprice_account.price_message.exponent.abs().try_into().unwrap()))
            .unwrap()
            .checked_div(pyth_solprice_account.price_message.price.try_into().unwrap())
            .unwrap();

        msg!("amount in lamports: {}", amount_in_lamports);

        // Check if the price of the asset exceeds the strike_price defined in the locker
        if amount_in_lamports >= *strike_price as u64 {
            // asset price exceeds strike_price so unlock the lock
            *locked = false;
            Ok(())
        } else {
            Err(LockerErrorCode::StrikePriceTooLow.into())
        }
    } else {
        Err(LockerErrorCode::NotAPriceLock.into())
    }
}


