use anchor_lang::prelude::*;

#[error_code]
pub enum ClmmError {
    #[msg("Token mints must be different")]
    IdenticalTokenMints,

    #[msg("Initial sqrt price must be greater than 0")]
    InvalidInitialPrice,

    #[msg("Tick spacing must be greater than 0")]
    InvalidTickSpacing,
}