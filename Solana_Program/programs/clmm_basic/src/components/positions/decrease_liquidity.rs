use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};
use crate::state::{Pool, Position};
use crate::libraries::tick_array::{TickArray, TICK_ARRAY_SIZE};
use crate::libraries::tick_math::is_tick_valid;
use crate::libraries::liquidity_math::amounts_from_liquidity;
use crate::errors::ClmmError;

#[derive(Accounts)]
#[instruction(liquidity_to_remove: u128, amount0_min: u64, amount1_min: u64, lower_tick: i32, upper_tick: i32, lower_tick_array_start: i32, upper_tick_array_start: i32)]
pub struct DecreaseLiquidity<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        constraint = position.initialized @ ClmmError::PositionNotInitialized,
        constraint = position.pool == pool.key() @ ClmmError::PositionPoolMismatch,
        constraint = position.owner == payer.key() @ ClmmError::PositionOwnerMismatch,
        constraint = position.liquidity >= liquidity_to_remove @ ClmmError::InvalidLiquidity,
        constraint = position.lower_tick == lower_tick @ ClmmError::InvalidTick,
        constraint = position.upper_tick == upper_tick @ ClmmError::InvalidTick,
    )]
    pub position: Account<'info, Position>,

    #[account(
        seeds = [b"pool", pool.token0_mint.as_ref(), pool.token1_mint.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        seeds = [
            b"tick_array",
            pool.key().as_ref(),
            &lower_tick_array_start.to_le_bytes()
        ],
        bump
    )]
    pub lower_tick_array: AccountLoader<'info, TickArray>,

    #[account(
        mut,
        seeds = [
            b"tick_array",
            pool.key().as_ref(),
            &upper_tick_array_start.to_le_bytes()
        ],
        bump
    )]
    pub upper_tick_array: AccountLoader<'info, TickArray>,

    #[account(
        mut,
        constraint = user_token0.mint == pool.token0_mint @ ClmmError::TokenAccountMintMismatch,
        constraint = user_token0.owner == payer.key() @ ClmmError::PositionOwnerMismatch,
    )]
    pub user_token0: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_token1.mint == pool.token1_mint @ ClmmError::TokenAccountMintMismatch,
        constraint = user_token1.owner == payer.key() @ ClmmError::PositionOwnerMismatch,
    )]
    pub user_token1: Account<'info, TokenAccount>,

    #[account(
        mut,
        address = pool.token0_vault
    )]
    pub pool_token0_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        address = pool.token1_vault
    )]
    pub pool_token1_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

fn get_tick_array_start_tick(tick: i32, tick_spacing: u16) -> i32 {
    let ticks_per_array = (TICK_ARRAY_SIZE as i32) * (tick_spacing as i32);
    let start_tick = (tick / ticks_per_array) * ticks_per_array;
    if tick < 0 && tick % ticks_per_array != 0 {
        start_tick - ticks_per_array
    } else {
        start_tick
    }
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
    let pool = &ctx.accounts.pool;
    let tick_spacing = pool.tick_spacing;
    validate_accounts(&ctx, liquidity_to_remove, lower_tick, upper_tick, lower_tick_array_start, upper_tick_array_start, tick_spacing)?;

    let (amount0, amount1) = compute_withdraw_amounts(&ctx, liquidity_to_remove)?;

    let mut lower_tick_array = ctx.accounts.lower_tick_array.load_mut()?;
    let mut upper_tick_array = ctx.accounts.upper_tick_array.load_mut()?;
    update_ticks(&mut lower_tick_array, &mut upper_tick_array, &ctx.accounts.position, liquidity_to_remove)?;

    clear_unused_ticks(&mut lower_tick_array, &mut upper_tick_array, &ctx.accounts.position)?;

    update_position(&mut ctx.accounts.position, liquidity_to_remove, amount0, amount1)?;

    update_pool(&mut ctx.accounts.pool, &ctx.accounts.position, liquidity_to_remove)?;

    perform_slippage_check(amount0, amount1, amount0_min, amount1_min)?;

    transfer_tokens(&ctx, amount0, amount1)?;

    Ok(())
}

fn validate_accounts(
    ctx: &Context<DecreaseLiquidity>,
    liquidity_to_remove: u128,
    lower_tick: i32,
    upper_tick: i32,
    lower_tick_array_start: i32,
    upper_tick_array_start: i32,
    tick_spacing: u16,
) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let position = &ctx.accounts.position;
    let lower_tick_array = ctx.accounts.lower_tick_array.load()?;
    let upper_tick_array = ctx.accounts.upper_tick_array.load()?;

    require!(is_tick_valid(lower_tick), ClmmError::InvalidTick);
    require!(is_tick_valid(upper_tick), ClmmError::InvalidTick);
    require!(lower_tick < upper_tick, ClmmError::InvalidPriceRange);

    require!(position.lower_tick == lower_tick, ClmmError::InvalidTick);
    require!(position.upper_tick == upper_tick, ClmmError::InvalidTick);

    let expected_lower_start = get_tick_array_start_tick(lower_tick, tick_spacing);
    let expected_upper_start = get_tick_array_start_tick(upper_tick, tick_spacing);

    require!(lower_tick_array_start == expected_lower_start, ClmmError::InvalidStartTick);
    require!(upper_tick_array_start == expected_upper_start, ClmmError::InvalidStartTick);

    require!(lower_tick_array.initialized == 1, ClmmError::TickArrayNotInitialized);
    require!(lower_tick_array.start_tick_index == lower_tick_array_start, ClmmError::InvalidStartTick);
    require!(lower_tick_array.tick_spacing == tick_spacing, ClmmError::InvalidTickSpacing);
    require!(lower_tick_array.pool == pool.key(), ClmmError::InvalidTickArrayPool);

    require!(upper_tick_array.initialized == 1, ClmmError::TickArrayNotInitialized);
    require!(upper_tick_array.start_tick_index == upper_tick_array_start, ClmmError::InvalidStartTick);
    require!(upper_tick_array.tick_spacing == tick_spacing, ClmmError::InvalidTickSpacing);
    require!(upper_tick_array.pool == pool.key(), ClmmError::InvalidTickArrayPool);

    require!(liquidity_to_remove > 0, ClmmError::InvalidLiquidity);
    require!(position.liquidity >= liquidity_to_remove, ClmmError::InvalidLiquidity);

    Ok(())
}

fn compute_withdraw_amounts(ctx: &Context<DecreaseLiquidity>, liquidity_to_remove: u128) -> Result<(u64, u64)> {
    let pool = &ctx.accounts.pool;
    let position = &ctx.accounts.position;

    let liquidity_delta = i128::try_from(liquidity_to_remove).map_err(|_| ClmmError::MathOverflow)?;
    let (amount0, amount1) = amounts_from_liquidity(
        pool.current_tick,
        pool.sqrt_price,
        position.lower_tick,
        position.upper_tick,
        -liquidity_delta,
    )?;

    Ok((amount0, amount1))
}

fn update_ticks(
    lower_tick_array: &mut TickArray,
    upper_tick_array: &mut TickArray,
    position: &Account<Position>,
    liquidity_delta: u128,
) -> Result<()> {
    let lower_tick = &mut lower_tick_array.get_tick_mut(position.lower_tick)?;
    let upper_tick = &mut upper_tick_array.get_tick_mut(position.upper_tick)?;

    let liquidity_delta_i128 = i128::try_from(liquidity_delta).map_err(|_| ClmmError::MathOverflow)?;
    let negative_liquidity_delta = -liquidity_delta_i128;

    lower_tick.update_tick(negative_liquidity_delta, false, false)?;
    upper_tick.update_tick(negative_liquidity_delta, true, false)?;

    Ok(())
}

fn clear_unused_ticks(
    lower_tick_array: &mut TickArray,
    upper_tick_array: &mut TickArray,
    position: &Account<Position>,
) -> Result<()> {
    let lower_tick = &mut lower_tick_array.get_tick_mut(position.lower_tick)?;
    let upper_tick = &mut upper_tick_array.get_tick_mut(position.upper_tick)?;

    if lower_tick.liquidity_gross == 0 {
        lower_tick.clear_tick()?;
    }
    if upper_tick.liquidity_gross == 0 {
        upper_tick.clear_tick()?;
    }

    Ok(())
}

fn update_position(
    position: &mut Account<Position>,
    liquidity_delta: u128,
    amount0: u64,
    amount1: u64,
) -> Result<()> {
    position.liquidity = position.liquidity.checked_sub(liquidity_delta).ok_or(ClmmError::MathOverflow)?;
    position.amount0 = position.amount0.checked_sub(amount0).ok_or(ClmmError::MathOverflow)?;
    position.amount1 = position.amount1.checked_sub(amount1).ok_or(ClmmError::MathOverflow)?;

    Ok(())
}

fn update_pool(
    pool: &mut Account<Pool>,
    position: &Account<Position>,
    liquidity_delta: u128,
) -> Result<()> {
    let in_range = pool.current_tick >= position.lower_tick && pool.current_tick < position.upper_tick;
    if in_range {
        pool.liquidity = pool.liquidity.checked_sub(liquidity_delta).ok_or(ClmmError::MathOverflow)?;
    }

    Ok(())
}

fn perform_slippage_check(
    amount0: u64,
    amount1: u64,
    amount0_min: u64,
    amount1_min: u64,
) -> Result<()> {
    require!(amount0 >= amount0_min, ClmmError::SlippageExceeded);
    require!(amount1 >= amount1_min, ClmmError::SlippageExceeded);
    Ok(())
}

fn transfer_tokens(
    ctx: &Context<DecreaseLiquidity>,
    amount0: u64,
    amount1: u64,
) -> Result<()> {
    let pool = &ctx.accounts.pool;

    if amount0 > 0 {
        let seeds = &[
            b"pool",
            pool.token0_mint.as_ref(),
            pool.token1_mint.as_ref(),
            &[pool.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_token0_vault.to_account_info(),
            to: ctx.accounts.user_token0.to_account_info(),
            authority: pool.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount0)?;
    }

    if amount1 > 0 {
        let seeds = &[
            b"pool",
            pool.token0_mint.as_ref(),
            pool.token1_mint.as_ref(),
            &[pool.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_token1_vault.to_account_info(),
            to: ctx.accounts.user_token1.to_account_info(),
            authority: pool.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount1)?;
    }

    Ok(())
}