use crate::errors::ClmmError;
use anchor_lang::prelude::*;

pub const MIN_TICK: i32 = -443636;
pub const MAX_TICK: i32 = 443636;

pub const Q64: u128 = 1u128 << 64;

pub const MIN_SQRT_PRICE_X64: u128 = 4295048016;
pub const MAX_SQRT_PRICE_X64: u128 = 79226673521066979257578248091u128;

const SQRT_1_0001_Q64: [u128; 16] = [
    0xfffcb933bd6fb800,
    0xfff97272373d4000,
    0xfff2e50f5f657000,
    0xffe5caca7e10f000,
    0xffcb9843d60f7000,
    0xff973b41fa98e800,
    0xff2ea16466c9b000,
    0xfe5dee046a9a3800,
    0xfcbe86c7900bb000,
    0xf987a7253ac65800,
    0xf3392b0822bb6000,
    0xe7159475a2caf000,
    0xd097f3bdfd2f2000,
    0xa9f746462d9f8000,
    0x70d869a156f31c00,
    0x31be135f97ed3200,
];

pub fn is_tick_valid(tick: i32) -> bool {
    tick >= MIN_TICK && tick <= MAX_TICK
}

pub fn align_tick_to_spacing(tick: i32, spacing: u16) -> Result<i32> {
    require!(spacing > 0, ClmmError::InvalidTickSpacing);

    let spacing = spacing as i32;
    let mut aligned = tick / spacing * spacing;

    if tick < 0 && tick % spacing != 0 {
        aligned -= spacing;
    }

    require!(is_tick_valid(aligned), ClmmError::InvalidTick);

    Ok(aligned)
}

pub fn min_tick() -> i32 {
    MIN_TICK
}

pub fn max_tick() -> i32 {
    MAX_TICK
}

pub fn min_sqrt_price_x64() -> u128 {
    MIN_SQRT_PRICE_X64
}

pub fn max_sqrt_price_x64() -> u128 {
    MAX_SQRT_PRICE_X64
}

pub fn tick_to_sqrt_price_x64(tick: i32) -> Result<u128> {
    require!(is_tick_valid(tick), ClmmError::InvalidTick);

    let abs_tick = tick.unsigned_abs();

    let mut ratio: u128 = if abs_tick & 0x1 != 0 {
        SQRT_1_0001_Q64[0]
    } else {
        Q64
    };

    if abs_tick & 0x2 != 0 {
        ratio = ratio
            .checked_mul(SQRT_1_0001_Q64[1])
            .ok_or(error!(ClmmError::MathOverflow))?
            >> 64;
    }
    if abs_tick & 0x4 != 0 {
        ratio = ratio
            .checked_mul(SQRT_1_0001_Q64[2])
            .ok_or(error!(ClmmError::MathOverflow))?
            >> 64;
    }
    if abs_tick & 0x8 != 0 {
        ratio = ratio
            .checked_mul(SQRT_1_0001_Q64[3])
            .ok_or(error!(ClmmError::MathOverflow))?
            >> 64;
    }
    if abs_tick & 0x10 != 0 {
        ratio = ratio
            .checked_mul(SQRT_1_0001_Q64[4])
            .ok_or(error!(ClmmError::MathOverflow))?
            >> 64;
    }
    if abs_tick & 0x20 != 0 {
        ratio = ratio
            .checked_mul(SQRT_1_0001_Q64[5])
            .ok_or(error!(ClmmError::MathOverflow))?
            >> 64;
    }
    if abs_tick & 0x40 != 0 {
        ratio = ratio
            .checked_mul(SQRT_1_0001_Q64[6])
            .ok_or(error!(ClmmError::MathOverflow))?
            >> 64;
    }
    if abs_tick & 0x80 != 0 {
        ratio = ratio
            .checked_mul(SQRT_1_0001_Q64[7])
            .ok_or(error!(ClmmError::MathOverflow))?
            >> 64;
    }
    if abs_tick & 0x100 != 0 {
        ratio = ratio
            .checked_mul(SQRT_1_0001_Q64[8])
            .ok_or(error!(ClmmError::MathOverflow))?
            >> 64;
    }
    if abs_tick & 0x200 != 0 {
        ratio = ratio
            .checked_mul(SQRT_1_0001_Q64[9])
            .ok_or(error!(ClmmError::MathOverflow))?
            >> 64;
    }
    if abs_tick & 0x400 != 0 {
        ratio = ratio
            .checked_mul(SQRT_1_0001_Q64[10])
            .ok_or(error!(ClmmError::MathOverflow))?
            >> 64;
    }
    if abs_tick & 0x800 != 0 {
        ratio = ratio
            .checked_mul(SQRT_1_0001_Q64[11])
            .ok_or(error!(ClmmError::MathOverflow))?
            >> 64;
    }
    if abs_tick & 0x1000 != 0 {
        ratio = ratio
            .checked_mul(SQRT_1_0001_Q64[12])
            .ok_or(error!(ClmmError::MathOverflow))?
            >> 64;
    }
    if abs_tick & 0x2000 != 0 {
        ratio = ratio
            .checked_mul(SQRT_1_0001_Q64[13])
            .ok_or(error!(ClmmError::MathOverflow))?
            >> 64;
    }
    if abs_tick & 0x4000 != 0 {
        ratio = ratio
            .checked_mul(SQRT_1_0001_Q64[14])
            .ok_or(error!(ClmmError::MathOverflow))?
            >> 64;
    }
    if abs_tick & 0x8000 != 0 {
        ratio = ratio
            .checked_mul(SQRT_1_0001_Q64[15])
            .ok_or(error!(ClmmError::MathOverflow))?
            >> 64;
    }

    if tick > 0 {
        ratio = u128::MAX / ratio;
    }

    Ok(ratio)
}

pub fn sqrt_price_x64_to_tick(sqrt_price_x64: u128) -> Result<i32> {
    require!(
        sqrt_price_x64 >= MIN_SQRT_PRICE_X64 && sqrt_price_x64 <= MAX_SQRT_PRICE_X64,
        ClmmError::InvalidSqrtPrice
    );

    let msb: u32 = 128 - sqrt_price_x64.leading_zeros() - 1;
    let log2p_integer_x32 = ((msb as i128 - 64) << 32);

    let mut bit: i128 = 0x8000_0000_0000_0000i128;
    let mut precision = 0;
    let mut log2p_fraction_x64 = 0i128;

    let mut r = if msb >= 64 {
        sqrt_price_x64 >> (msb - 63)
    } else {
        sqrt_price_x64 << (63 - msb)
    };

    const BIT_PRECISION: u32 = 16;

    while bit > 0 && precision < BIT_PRECISION {
        r = r
            .checked_mul(r)
            .ok_or(ClmmError::MathOverflow)?;

        let is_r_more_than_two = (r >> 127) as u32;
        r >>= 63 + is_r_more_than_two;

        log2p_fraction_x64 = log2p_fraction_x64
            .checked_add(bit * is_r_more_than_two as i128)
            .ok_or(error!(ClmmError::MathOverflow))?;
        bit >>= 1;
        precision += 1;
    }

    let log2p_fraction_x32 = log2p_fraction_x64 >> 32;
    let log2p_x32 = log2p_integer_x32
        .checked_add(log2p_fraction_x32)
        .ok_or(error!(ClmmError::MathOverflow))?;

    let log_sqrt_10001_x64 = log2p_x32
        .checked_mul(59543866431248i128)
        .ok_or(error!(ClmmError::MathOverflow))?;

    let tick_low = ((log_sqrt_10001_x64
        .checked_sub(184467440737095516i128)
        .ok_or(error!(ClmmError::MathOverflow))?) >> 64) as i32;

    let tick_high = ((log_sqrt_10001_x64
        .checked_add(15793534762490258745i128)
        .ok_or(error!(ClmmError::MathOverflow))?) >> 64) as i32;

    if tick_low == tick_high {
        Ok(tick_low)
    } else if tick_to_sqrt_price_x64(tick_high)? <= sqrt_price_x64 {
        Ok(tick_high)
    } else {
        Ok(tick_low)
    }
}

pub fn sqrt_price_to_tick(sqrt_price: u128) -> Result<i32> {
    sqrt_price_x64_to_tick(sqrt_price)
}

pub fn tick_to_sqrt_price(tick: i32) -> Result<u128> {
    tick_to_sqrt_price_x64(tick)
}