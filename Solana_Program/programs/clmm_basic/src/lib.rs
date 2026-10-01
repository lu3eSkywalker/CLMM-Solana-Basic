use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
mod components;
pub use components::create_pool::CreatePool;
mod libraries;

mod utils;
use utils::token_vault::create_token_vault_account;

mod errors;
pub use errors::*;

mod state;
pub use state::*;

pub use components::positions::increase_liquidity::IncreaseLiquidity;
pub use components::positions::decrease_liquidity::DecreaseLiquidity;

// Anchor's #[program] macro generates `pub use crate::__client_accounts_*::*`
// inside its `accounts` module, so these derived helper mods must be
// resolvable from the crate root.
pub(crate) use components::create_pool::__client_accounts_create_pool;
pub(crate) use components::positions::open_position::__client_accounts_open_position;
pub(crate) use components::positions::increase_liquidity::__client_accounts_increase_liquidity;
pub(crate) use components::positions::decrease_liquidity::__client_accounts_decrease_liquidity;

declare_id!("BrZPVRu7HgWe9yZM3XiDx9qRdbXwR8RBFW4JiyanQm75");

#[program]
pub mod clmm_basic {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreatePool>,
        token0_mint: Pubkey,
        token1_mint: Pubkey,
        initial_sqrt_price: u128,
        tick_spacing: u16,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let bump = ctx.bumps.pool;

        require!(token0_mint != token1_mint, ClmmError::IdenticalTokenMints);
        require!(tick_spacing > 0, ClmmError::InvalidTickSpacing);
        require!(initial_sqrt_price > 0, ClmmError::InvalidInitialPrice);

        let current_tick = 0i32;

        let (token0_mint, token1_mint) = if token0_mint < token1_mint {
            (token0_mint, token1_mint)
        } else {
            (token1_mint, token0_mint)
        };

        pool.token0_mint = token0_mint;
        pool.token1_mint = token1_mint;

        let pool_pda = pool.to_account_info();

        create_token_vault_account(
            &mut ctx.accounts.token0_vault.to_account_info(),
            &ctx.accounts.token0_mint.to_account_info(),
            &pool_pda,
            &ctx.accounts.payer.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
            &ctx.accounts.token_program.to_account_info(),
            &ctx.accounts.rent,
        )?;

        create_token_vault_account(
            &mut ctx.accounts.token1_vault.to_account_info(),
            &ctx.accounts.token1_mint.to_account_info(),
            &pool_pda,
            &ctx.accounts.payer.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
            &ctx.accounts.token_program.to_account_info(),
            &ctx.accounts.rent,
        )?;

        pool.token0_vault = ctx.accounts.token0_vault.key();
        pool.token1_vault = ctx.accounts.token1_vault.key();
        pool.sqrt_price = initial_sqrt_price;
        pool.current_tick = current_tick;
        pool.liquidity = 0;
        pool.tick_spacing = tick_spacing;
        pool.bump = bump;

        Ok(())
    }

    pub fn increase_liquidity(
        ctx: Context<IncreaseLiquidity>,
        amount0_max: u64,
        amount1_max: u64,
        lower_tick: i32,
        upper_tick: i32,
        lower_tick_array_start: i32,
        upper_tick_array_start: i32,
    ) -> Result<()> {
        components::positions::increase_liquidity::increase_liquidity(
            ctx,
            amount0_max,
            amount1_max,
            lower_tick,
            upper_tick,
            lower_tick_array_start,
            upper_tick_array_start,
        )
    }

    pub fn decrease_liquidity(
        ctx: Context<DecreaseLiquidity>,
        liquidity_to_remove: u128,
        amount0_min: u64,
        amount1_min: u64,
        lower_tick: i32,
        upper_tick: i32,
        lower_tick_array_start: i32,
        upper_tick_array_start: i32,
    ) -> Result<()> {
        components::positions::decrease_liquidity::decrease_liquidity(
            ctx,
            liquidity_to_remove,
            amount0_min,
            amount1_min,
            lower_tick,
            upper_tick,
            lower_tick_array_start,
            upper_tick_array_start,
        )
    }
}