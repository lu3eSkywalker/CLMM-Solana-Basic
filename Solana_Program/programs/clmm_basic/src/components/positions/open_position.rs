use anchor_lang::prelude::*;
use crate::state::{Pool, Position};
use crate::libraries::tick_array::{TickArray, TICK_ARRAY_SIZE};
use crate::libraries::tick_math::{is_tick_valid, align_tick_to_spacing};
use crate::errors::ClmmError;

#[derive(Accounts)]
#[instruction(lower_tick: i32, upper_tick: i32, lower_tick_array_start: i32, upper_tick_array_start: i32)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [b"pool", pool.token0_mint.as_ref(), pool.token1_mint.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer = payer,
        space = 8 + Position::INIT_SPACE,
        seeds = [
            b"position",
            pool.key().as_ref(),
            payer.key().as_ref(),
            &lower_tick.to_le_bytes(),
            &upper_tick.to_le_bytes()
        ],
        bump
    )]
    pub position: Account<'info, Position>,

    #[account(
        seeds = [
            b"tick_array",
            pool.key().as_ref(),
            &lower_tick_array_start.to_le_bytes()
        ],
        bump
    )]
    pub lower_tick_array: AccountLoader<'info, TickArray>,

    #[account(
        seeds = [
            b"tick_array",
            pool.key().as_ref(),
            &upper_tick_array_start.to_le_bytes()
        ],
        bump
    )]
    pub upper_tick_array: AccountLoader<'info, TickArray>,

    pub system_program: Program<'info, System>,
}

pub const POSITION_SEED: &[u8] = b"position";

pub fn get_tick_array_start_tick(tick: i32, tick_spacing: u16) -> i32 {
    let ticks_per_array = (TICK_ARRAY_SIZE as i32) * (tick_spacing as i32);
    let start_tick = (tick / ticks_per_array) * ticks_per_array;
    if tick < 0 && tick % ticks_per_array != 0 {
        start_tick - ticks_per_array
    } else {
        start_tick
    }
}

pub fn open_position(
    ctx: Context<OpenPosition>,
    lower_tick: i32,
    upper_tick: i32,
    lower_tick_array_start: i32,
    upper_tick_array_start: i32,
) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let tick_spacing = pool.tick_spacing;

    require!(is_tick_valid(lower_tick), ClmmError::InvalidTick);
    require!(is_tick_valid(upper_tick), ClmmError::InvalidTick);
    require!(lower_tick < upper_tick, ClmmError::InvalidPriceRange);

    let aligned_lower = align_tick_to_spacing(lower_tick, tick_spacing)?;
    let aligned_upper = align_tick_to_spacing(upper_tick, tick_spacing)?;

    require!(aligned_lower == lower_tick, ClmmError::TickNotAligned);
    require!(aligned_upper == upper_tick, ClmmError::TickNotAligned);

    let expected_lower_start = get_tick_array_start_tick(lower_tick, tick_spacing);
    let expected_upper_start = get_tick_array_start_tick(upper_tick, tick_spacing);

    require!(lower_tick_array_start == expected_lower_start, ClmmError::InvalidStartTick);
    require!(upper_tick_array_start == expected_upper_start, ClmmError::InvalidStartTick);

    let lower_tick_array = ctx.accounts.lower_tick_array.load()?;
    require!(lower_tick_array.initialized == 1, ClmmError::TickArrayNotInitialized);
    require!(lower_tick_array.start_tick_index == lower_tick_array_start, ClmmError::InvalidStartTick);
    require!(lower_tick_array.tick_spacing == tick_spacing, ClmmError::InvalidTickSpacing);
    require!(lower_tick_array.pool == pool.key(), ClmmError::InvalidTickArrayPool);

    let upper_tick_array = ctx.accounts.upper_tick_array.load()?;
    require!(upper_tick_array.initialized == 1, ClmmError::TickArrayNotInitialized);
    require!(upper_tick_array.start_tick_index == upper_tick_array_start, ClmmError::InvalidStartTick);
    require!(upper_tick_array.tick_spacing == tick_spacing, ClmmError::InvalidTickSpacing);
    require!(upper_tick_array.pool == pool.key(), ClmmError::InvalidTickArrayPool);

    let position = &mut ctx.accounts.position;
    position.initialize(
        ctx.accounts.payer.key(),
        pool.key(),
        lower_tick,
        upper_tick,
    )?;

    Ok(())
}