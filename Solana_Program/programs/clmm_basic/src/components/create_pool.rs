use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use crate::state::Pool;

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