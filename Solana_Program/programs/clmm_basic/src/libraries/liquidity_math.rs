use anchor_lang::prelude::*;
use crate::errors::ClmmError;
use crate::libraries::tick_math::tick_to_sqrt_price_x64;
use crate::libraries::big_num::U256;
use crate::libraries::math_utils::{UnsafeMathTrait, U256MathExt};

pub const Q64: u128 = 1u128 << 64;
pub const RESOLUTION: u8 = 64;

pub fn liquidity_from_amounts(
    current_sqrt_price_x64: u128,
    lower_sqrt_price_x64: u128,
    upper_sqrt_price_x64: u128,
    amount0: u64,
    amount1: u64,
) -> Result<u128> {
    let liquidity_0: u128;
    let liquidity_1: u128;

    if current_sqrt_price_x64 <= lower_sqrt_price_x64 {
        liquidity_0 = liquidity_from_amount0(
            lower_sqrt_price_x64,
            upper_sqrt_price_x64,
            amount0,
        )?;
        liquidity_1 = 0;
    } else if current_sqrt_price_x64 < upper_sqrt_price_x64 {
        liquidity_0 = liquidity_from_amount0(
            current_sqrt_price_x64,
            upper_sqrt_price_x64,
            amount0,
        )?;
        liquidity_1 = liquidity_from_amount1(
            lower_sqrt_price_x64,
            current_sqrt_price_x64,
            amount1,
        )?;
    } else {
        liquidity_0 = 0;
        liquidity_1 = liquidity_from_amount1(
            lower_sqrt_price_x64,
            upper_sqrt_price_x64,
            amount1,
        )?;
    }

    if liquidity_0 > 0 && liquidity_1 > 0 {
        Ok(std::cmp::min(liquidity_0, liquidity_1))
    } else if liquidity_0 > 0 {
        Ok(liquidity_0)
    } else {
        Ok(liquidity_1)
    }
}

fn liquidity_from_amount0(
    sqrt_price_lower_x64: u128,
    sqrt_price_upper_x64: u128,
    amount0: u64,
) -> Result<u128> {
    if amount0 == 0 {
        return Ok(0);
    }

    let numerator = U256::from(amount0) * U256::from(sqrt_price_lower_x64) * U256::from(sqrt_price_upper_x64);
    let denominator = U256::from(sqrt_price_upper_x64 - sqrt_price_lower_x64) << RESOLUTION;

    let liquidity = U256::div_rounding_up(numerator, denominator);

    if liquidity > U256::from(u128::MAX) {
        return Err(ClmmError::MathOverflow.into());
    }

    Ok(liquidity.as_u128())
}

fn liquidity_from_amount1(
    sqrt_price_lower_x64: u128,
    sqrt_price_upper_x64: u128,
    amount1: u64,
) -> Result<u128> {
    if amount1 == 0 {
        return Ok(0);
    }

    let numerator = U256::from(amount1) * U256::from(Q64);
    let denominator = U256::from(sqrt_price_upper_x64 - sqrt_price_lower_x64);

    let liquidity = U256::div_rounding_up(numerator, denominator);

    if liquidity > U256::from(u128::MAX) {
        return Err(ClmmError::MathOverflow.into());
    }

    Ok(liquidity.as_u128())
}

pub fn apply_liquidity_delta(
    current_liquidity: u128,
    liquidity_delta: i128,
) -> Result<u128> {
    let z: u128;
    if liquidity_delta < 0 {
        z = current_liquidity - u128::try_from(-liquidity_delta).unwrap();
        require_gt!(current_liquidity, z, ClmmError::LiquiditySubValueError);
    } else {
        z = current_liquidity + u128::try_from(liquidity_delta).unwrap();
        require_gte!(z, current_liquidity, ClmmError::LiquidityAddValueError);
    }

    Ok(z)
}

pub fn amount0_from_liquidity(
    mut sqrt_price_lower_x64: u128,
    mut sqrt_price_upper_x64: u128,
    liquidity: u128,
    round_up: bool,
) -> Result<u64> {
    if sqrt_price_lower_x64 > sqrt_price_upper_x64 {
        std::mem::swap(&mut sqrt_price_lower_x64, &mut sqrt_price_upper_x64);
    };

    let numerator_1 = U256::from(liquidity) << RESOLUTION;
    let numerator_2 = U256::from(sqrt_price_upper_x64 - sqrt_price_lower_x64);

    assert!(sqrt_price_lower_x64 > 0);

    let result = if round_up {
        UnsafeMathTrait::div_rounding_up(
            numerator_1
                .mul_div_ceil(numerator_2, U256::from(sqrt_price_upper_x64))
                .unwrap(),
            U256::from(sqrt_price_lower_x64),
        )
    } else {
        numerator_1
            .mul_div_floor(numerator_2, U256::from(sqrt_price_upper_x64))
            .unwrap()
            / U256::from(sqrt_price_lower_x64)
    };
    if result > U256::from(u64::MAX) {
        return Err(ClmmError::MaxTokenOverflow.into());
    }
    return Ok(result.as_u64());
}

pub fn amount1_from_liquidity(
    mut sqrt_price_lower_x64: u128,
    mut sqrt_price_upper_x64: u128,
    liquidity: u128,
    round_up: bool,
) -> Result<u64> {
    if sqrt_price_lower_x64 > sqrt_price_upper_x64 {
        std::mem::swap(&mut sqrt_price_lower_x64, &mut sqrt_price_upper_x64);
    };

    let result = if round_up {
        U256::from(liquidity).mul_div_ceil(
            U256::from(sqrt_price_upper_x64 - sqrt_price_lower_x64),
            U256::from(Q64),
        )
    } else {
        U256::from(liquidity).mul_div_floor(
            U256::from(sqrt_price_upper_x64 - sqrt_price_lower_x64),
            U256::from(Q64),
        )
    }.unwrap();
    if result > U256::from(u64::MAX) {
        return Err(ClmmError::MaxTokenOverflow.into());
    }
    return Ok(result.as_u64());
}

pub fn amount0_from_liquidity_signed(
    sqrt_price_lower_x64: u128,
    sqrt_price_upper_x64: u128,
    liquidity_delta: i128,
) -> Result<u64> {
    if liquidity_delta < 0 {
        amount0_from_liquidity(
            sqrt_price_lower_x64,
            sqrt_price_upper_x64,
            u128::try_from(-liquidity_delta).unwrap(),
            false,
        )
    } else {
        amount0_from_liquidity(
            sqrt_price_lower_x64,
            sqrt_price_upper_x64,
            u128::try_from(liquidity_delta).unwrap(),
            true,
        )
    }
}

pub fn amount1_from_liquidity_signed(
    sqrt_price_lower_x64: u128,
    sqrt_price_upper_x64: u128,
    liquidity_delta: i128,
) -> Result<u64> {
    if liquidity_delta < 0 {
        amount1_from_liquidity(
            sqrt_price_lower_x64,
            sqrt_price_upper_x64,
            u128::try_from(-liquidity_delta).unwrap(),
            false,
        )
    } else {
        amount1_from_liquidity(
            sqrt_price_lower_x64,
            sqrt_price_upper_x64,
            u128::try_from(liquidity_delta).unwrap(),
            true,
        )
    }
}

pub fn amounts_from_liquidity(
    current_tick: i32,
    current_sqrt_price_x64: u128,
    lower_tick: i32,
    upper_tick: i32,
    liquidity_delta: i128,
) -> Result<(u64, u64)> {
    let mut amount_0 = 0;
    let mut amount_1 = 0;

    if current_tick < lower_tick {
        amount_0 = amount0_from_liquidity_signed(
            tick_to_sqrt_price_x64(lower_tick)?,
            tick_to_sqrt_price_x64(upper_tick)?,
            liquidity_delta,
        )?;
    } else if current_tick < upper_tick {
        amount_0 = amount0_from_liquidity_signed(
            current_sqrt_price_x64,
            tick_to_sqrt_price_x64(upper_tick)?,
            liquidity_delta,
        )?;
        amount_1 = amount1_from_liquidity_signed(
            tick_to_sqrt_price_x64(lower_tick)?,
            current_sqrt_price_x64,
            liquidity_delta,
        )?;
    } else {
        amount_1 = amount1_from_liquidity_signed(
            tick_to_sqrt_price_x64(lower_tick)?,
            tick_to_sqrt_price_x64(upper_tick)?,
            liquidity_delta,
        )?;
    }
    Ok((amount_0, amount_1))
}