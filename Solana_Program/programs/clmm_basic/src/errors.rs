use anchor_lang::prelude::*;

#[error_code]
pub enum ClmmError {
    #[msg("Token mints must be different")]
    IdenticalTokenMints,

    #[msg("Initial sqrt price must be greater than 0")]
    InvalidInitialPrice,

    #[msg("Tick spacing must be greater than 0")]
    InvalidTickSpacing,

    #[msg("Initial tick is not aligned with tick spacing")]
    InvalidInitialTick,

    #[msg("Tick out of bounds")]
    InvalidTick,

    #[msg("Invalid sqrt price")]
    InvalidSqrtPrice,

    #[msg("Invalid price range")]
    InvalidPriceRange,

    #[msg("Math overflow")]
    MathOverflow,

    #[msg("Tick already initialized")]
    TickAlreadyInitialized,

    #[msg("Tick not initialized")]
    TickNotInitialized,
}