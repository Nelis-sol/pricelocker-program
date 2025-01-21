use anchor_lang::prelude::*;


#[error_code]
pub enum LockerErrorCode {
    #[msg("Payout amount should be more than 0")]
    PayoutAmountNotPositive,
    #[msg("The found lock is not a price lock")]
    NotAPriceLock,
    #[msg("The lock does not exist on the index")]
    NoLockAtIndex,
    #[msg("Strike price is lower than the current token price")]
    StrikePriceTooLow,
    #[msg("The current time has not exceeded the strike time")]
    TimeLowerThanStrikeTime,
    #[msg("The found lock is not a time lock")]
    NotATimeLock,
    #[msg("The payout amount exceeds the available balance")]
    PayoutAmountExceedsAvailableBalance,
    #[msg("The pyth price feed account does not match the price feed in the lock.")]
    PriceFeedNotMatchingPricefeedAccount,
}


#[error_code]
pub enum PythErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("The config has already been initialized.")]
    ReInitialize,
    #[msg("The config has not been initialized.")]
    UnInitialize,
    #[msg("Argument is invalid.")]
    InvalidArgument,
    #[msg("An overflow occurs.")]
    Overflow,
    #[msg("Pyth has an internal error.")]
    PythError,
    #[msg("Pyth price oracle is offline.")]
    PythOffline,
    #[msg("The loan value is higher than the collateral value.")]
    LoanValueTooHigh,
    #[msg("Program should not try to serialize a price account.")]
    TryToSerializePriceAccount,
}
