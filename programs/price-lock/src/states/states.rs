use crate::*;

#[account]
pub struct Locker {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub creation_ts: u32,
    pub locked_balance: u32,
    pub locks: Vec<Lock>,
    pub locked: bool,
    pub bump: u8,
    pub name: String,
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
// added the initspace macro here against ChatGPT's advice, but seems necessary
// look here if the account structure is throwing errors
pub enum Lock {
    TimeLock {
        id: u8,
        strike_time: u32,
        amount: u32,
        locked: bool,
        join: Option<u8>,
    },
    PriceLock {
        id: u8,
        strike_price: u32,
        amount: u32,
        locked: bool,
        price_feed: Pubkey,
        join: Option<u8>,
    },
}
