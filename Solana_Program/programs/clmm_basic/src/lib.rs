use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

mod utils;
use utils::token_vault::create_token_vault_account;

mod errors;
pub use errors::*;

mod state;
pub use state::*;

declare_id!("EzsJ9BEKsa161p4iYoWymzzYY8gnd5uWqAnprekyrMDt");

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

#[derive(Accounts)]
#[instruction(token0_mint: Pubkey, token1_mint: Pubkey, initial_sqrt_price: u128, tick_spacing: u16)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + Pool::INIT_SPACE,
        seeds = [b"pool", token0_mint.key().as_ref(), token1_mint.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    /// CHECK: Created and initialized via create_token_vault_account helper
    pub token0_vault: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: Created and initialized via create_token_vault_account helper
    pub token1_vault: AccountInfo<'info>,

    pub token0_mint: Account<'info, Mint>,
    pub token1_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}