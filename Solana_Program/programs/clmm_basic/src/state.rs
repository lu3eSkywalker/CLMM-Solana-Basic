use anchor_lang::prelude::*;
use crate::errors::ClmmError;

#[account]
#[derive(InitSpace)]
pub struct Position {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub lower_tick: i32,
    pub upper_tick: i32,
    pub liquidity: u128,
    pub amount0: u64,
    pub amount1: u64,
    pub initialized: bool,
}

impl Position {
    pub fn initialize(
        &mut self,
        owner: Pubkey,
        pool: Pubkey,
        lower_tick: i32,
        upper_tick: i32,
    ) -> Result<()> {
        require!(!self.initialized, ClmmError::PositionAlreadyInitialized);
        self.owner = owner;
        self.pool = pool;
        self.lower_tick = lower_tick;
        self.upper_tick = upper_tick;
        self.liquidity = 0;
        self.amount0 = 0;
        self.amount1 = 0;
        self.initialized = true;
        Ok(())
    }
}

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