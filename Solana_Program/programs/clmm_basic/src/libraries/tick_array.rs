use anchor_lang::prelude::*;
use crate::libraries::tick::Tick;
use crate::errors::ClmmError;

pub const TICK_ARRAY_SIZE: usize = 88;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug, PartialEq, InitSpace)]
pub struct TickArray {
    pub start_tick_index: i32,
    pub tick_spacing: u16,
    pub initialized: bool,
    pub pool: Pubkey,
    pub ticks: [Tick; TICK_ARRAY_SIZE],
}

impl TickArray {
    pub fn initialize_tick_array(&mut self, start_tick_index: i32, tick_spacing: u16, pool: Pubkey) -> Result<()> {
        require!(!self.initialized, ClmmError::TickArrayAlreadyInitialized);
        require!(tick_spacing > 0, ClmmError::InvalidTickSpacing);
        require!(
            start_tick_index % tick_spacing as i32 == 0,
            ClmmError::InvalidStartTick
        );

        self.start_tick_index = start_tick_index;
        self.tick_spacing = tick_spacing;
        self.pool = pool;
        self.initialized = true;

        for tick in self.ticks.iter_mut() {
            tick.initialized = false;
            tick.liquidity_net = 0;
            tick.liquidity_gross = 0;
            tick.index = 0;
        }

        Ok(())
    }

    fn validate_tick(&self, tick_index: i32) -> Result<usize> {
        require!(self.initialized, ClmmError::TickArrayNotInitialized);
        require!(
            tick_index % self.tick_spacing as i32 == 0,
            ClmmError::TickNotAligned
        );

        let offset = tick_index
            .checked_sub(self.start_tick_index)
            .ok_or(ClmmError::ArithmeticOverflow)?;

        let array_index = offset
            .checked_div(self.tick_spacing as i32)
            .ok_or(ClmmError::ArithmeticOverflow)?;

        require!(array_index >= 0, ClmmError::TickOutOfRange);
        require!(
            (array_index as usize) < TICK_ARRAY_SIZE,
            ClmmError::TickOutOfRange
        );

        Ok(array_index as usize)
    }

    pub fn get_tick(&self, tick_index: i32) -> Result<&Tick> {
        let array_index = self.validate_tick(tick_index)?;
        Ok(&self.ticks[array_index])
    }

    pub fn get_tick_mut(&mut self, tick_index: i32) -> Result<&mut Tick> {
        let array_index = self.validate_tick(tick_index)?;
        Ok(&mut self.ticks[array_index])
    }

    pub fn find_tick(&self, tick_index: i32) -> Option<usize> {
        if !self.initialized {
            return None;
        }

        if tick_index % self.tick_spacing as i32 != 0 {
            return None;
        }

        let offset = tick_index.checked_sub(self.start_tick_index)?;
        let array_index = offset.checked_div(self.tick_spacing as i32)?;

        if array_index < 0 || array_index >= TICK_ARRAY_SIZE as i32 {
            return None;
        }

        Some(array_index as usize)
    }

    pub fn next_initialized_tick(
        &self,
        current_tick: i32,
        forward: bool,
    ) -> Result<Option<(i32, &Tick)>> {
        require!(self.initialized, ClmmError::TickArrayNotInitialized);
        require!(
            self.find_tick(current_tick).is_some(),
            ClmmError::TickOutOfRange
        );

        let start_index = self.find_tick(current_tick).unwrap();

        if forward {
            for i in (start_index + 1)..TICK_ARRAY_SIZE {
                if self.ticks[i].initialized {
                    let tick_index = self.start_tick_index + (i as i32 * self.tick_spacing as i32);
                    return Ok(Some((tick_index, &self.ticks[i])));
                }
            }
        } else {
            for i in (0..start_index).rev() {
                if self.ticks[i].initialized {
                    let tick_index = self.start_tick_index + (i as i32 * self.tick_spacing as i32);
                    return Ok(Some((tick_index, &self.ticks[i])));
                }
            }
        }

        Ok(None)
    }
}