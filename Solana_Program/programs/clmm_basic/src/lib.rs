use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
mod components;
pub use components::create_pool::CreatePool;
pub(crate) use components::create_pool::__client_accounts_create_pool;

mod utils;
use utils::token_vault::create_token_vault_account;

mod errors;
pub use errors::*;

mod state;
pub use state::*;

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
}