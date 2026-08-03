use anchor_lang::prelude::*;
use crate::errors::ClmmError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug, PartialEq, InitSpace)]
pub struct Tick {
    pub index: i32,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub initialized: bool,
}

impl Tick {
    pub fn initialize_tick(&mut self, index: i32) -> Result<()> {
        require!(!self.initialized, ClmmError::TickAlreadyInitialized);
        require!(crate::libraries::tick_math::is_tick_valid(index), ClmmError::InvalidTick);

        self.index = index;
        self.liquidity_net = 0;
        self.liquidity_gross = 0;
        self.initialized = true;

        Ok(())
    }

    pub fn update_tick(&mut self, liquidity_delta: i128, upper: bool, add: bool) -> Result<bool> {
        require!(self.initialized, ClmmError::TickNotInitialized);
        require!(liquidity_delta > 0, ClmmError::InvalidLiquidity);

        let delta = liquidity_delta.unsigned_abs();

        if add {
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

        let signed_delta = if add {
            liquidity_delta
        } else {
            -liquidity_delta
        };

        if upper {
            self.liquidity_net = self
                .liquidity_net
                .checked_sub(signed_delta)
                .ok_or(ClmmError::MathOverflow)?;
        } else {
            self.liquidity_net = self
                .liquidity_net
                .checked_add(signed_delta)
                .ok_or(ClmmError::MathOverflow)?;
        }

        let can_clear = self.liquidity_gross == 0;
        Ok(can_clear)
    }

    pub fn cross_tick(&self) -> Result<i128> {
        require!(self.initialized, ClmmError::TickNotInitialized);
        Ok(self.liquidity_net)
    }

    pub fn clear_tick(&mut self) -> Result<()> {
        require!(self.initialized, ClmmError::TickNotInitialized);
        require!(self.liquidity_gross == 0, ClmmError::TickStillInUse);

        self.index = 0;
        self.liquidity_net = 0;
        self.liquidity_gross = 0;
        self.initialized = false;

        Ok(())
    }
}