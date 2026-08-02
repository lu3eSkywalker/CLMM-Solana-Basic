use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    // Token information
    pub token0_mint: Pubkey,
    pub token1_mint: Pubkey,

    // Token vaults
    pub token0_vault: Pubkey,
    pub token1_vault: Pubkey,

    // Price state
    pub sqrt_price: u128,
    pub current_tick: i32,

    // Liquidity state
    pub liquidity: u128,

    // Configuration
    pub tick_spacing: u16,

    // PDA bump
    pub bump: u8,
}