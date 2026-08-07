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

    #[msg("Liquidity overflow")]
    LiquidityOverflow,

    #[msg("Liquidity underflow")]
    LiquidityUnderflow,

    #[msg("Tick still in use")]
    TickStillInUse,
#[msg("Invalid liquidity amount")]
    InvalidLiquidity,
    #[msg("Liquidity subtraction value error")]
    LiquiditySubValueError,
    #[msg("Liquidity addition value error")]
    LiquidityAddValueError,
    #[msg("Max token overflow")]
    MaxTokenOverflow,
    #[msg("Position already initialized")]
    PositionAlreadyInitialized,
    #[msg("Tick array belongs to a different pool")]
    InvalidTickArrayPool,
    #[msg("Slippage exceeded")]
    SlippageExceeded,
    #[msg("Position not initialized")]
    PositionNotInitialized,
    #[msg("Position does not belong to pool")]
    PositionPoolMismatch,
    #[msg("Position owner mismatch")]
    PositionOwnerMismatch,
    #[msg("Token account mint mismatch")]
    TokenAccountMintMismatch,
}