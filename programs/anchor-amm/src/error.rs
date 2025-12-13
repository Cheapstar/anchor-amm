use anchor_lang::error_code;




#[error_code]
pub enum AmmError {
    #[msg("Market is Locked")]
    MarketLocked,
    #[msg("Invalid Amount")]
    InvalidAmount,
}