use anchor_lang::prelude::*;
use crate::errors::ClmmError;
use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Default, Debug, PartialEq, Pod, Zeroable)]
#[repr(C, packed)]
pub struct Tick {
    pub liquidity_gross: u128,
    pub liquidity_net: i128,
    pub index: i32,
    pub initialized: u8,
}

impl anchor_lang::IdlBuild for Tick {}

impl anchor_lang::Space for Tick {
    const INIT_SPACE: usize = 37; // 16 + 16 + 4 + 1 = 37 bytes (no padding with packed)
}

impl Tick {
    pub fn initialize_tick(&mut self, index: i32) -> Result<()> {
        require!(self.initialized == 0, ClmmError::TickAlreadyInitialized);
        require!(crate::libraries::tick_math::is_tick_valid(index), ClmmError::InvalidTick);

        self.index = index;
        self.liquidity_net = 0;
        self.liquidity_gross = 0;
        self.initialized = 1;

        Ok(())
    }

    pub fn update_tick(&mut self, liquidity_delta: i128, upper: bool, _add: bool) -> Result<bool> {
        require!(self.initialized == 1, ClmmError::TickNotInitialized);
        require!(liquidity_delta != 0, ClmmError::InvalidLiquidity);

        let delta = liquidity_delta.unsigned_abs();
        let is_add = liquidity_delta > 0;

        if is_add {
            self.liquidity_gross = self
                .liquidity_gross
                .checked_add(delta)
                .ok_or(ClmmError::MathOverflow)?;
        } else {
            self.liquidity_gross = self
                .liquidity_gross
                .checked_sub(delta)
                .ok_or(ClmmError::MathOverflow)?;
        }

        if upper {
            self.liquidity_net = self
                .liquidity_net
                .checked_sub(liquidity_delta)
                .ok_or(ClmmError::MathOverflow)?;
        } else {
            self.liquidity_net = self
                .liquidity_net
                .checked_add(liquidity_delta)
                .ok_or(ClmmError::MathOverflow)?;
        }

        let can_clear = self.liquidity_gross == 0;
        Ok(can_clear)
    }

    pub fn cross_tick(&self) -> Result<i128> {
        require!(self.initialized == 1, ClmmError::TickNotInitialized);
        Ok(self.liquidity_net)
    }

    pub fn clear_tick(&mut self) -> Result<()> {
        require!(self.initialized == 1, ClmmError::TickNotInitialized);
        require!(self.liquidity_gross == 0, ClmmError::TickStillInUse);

        self.index = 0;
        self.liquidity_net = 0;
        self.liquidity_gross = 0;
        self.initialized = 0;

        Ok(())
    }
}