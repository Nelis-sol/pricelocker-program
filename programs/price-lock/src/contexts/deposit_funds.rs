use crate::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};


// Deposit funds to the locker_token_account, of which the locker_pda is the authority
#[derive(Accounts)]
#[instruction(locker_name: String)]
pub struct DepositNewFunds<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,
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
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}


impl<'info> DepositNewFunds<'_> {
    pub fn process(&mut self, amount: u64) -> Result<()> {
        let Self {authority, locker_token_account, token_program, authority_token_account,..} = self;
        
        // Transfer SPL tokens from authority's token account to the locker token account
        anchor_spl::token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: authority_token_account.to_account_info(),
                    to: locker_token_account.to_account_info(),
                    authority: authority.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }
}


