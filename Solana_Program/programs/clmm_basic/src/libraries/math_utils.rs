// Generic arithmetic utilities for fixed-point math

use crate::libraries::big_num::U256;

pub trait UnsafeMathTrait {
    fn div_rounding_up(x: Self, y: Self) -> Self;
}

impl UnsafeMathTrait for U256 {
    fn div_rounding_up(x: Self, y: Self) -> Self {
        x / y + U256::from((x % y > U256::default()) as u8)
    }
}

pub trait U256MathExt {
    fn mul_div_floor(self, num: U256, denom: U256) -> Option<U256>;
    fn mul_div_ceil(self, num: U256, denom: U256) -> Option<U256>;
}

impl U256MathExt for U256 {
    fn mul_div_floor(self, num: U256, denom: U256) -> Option<U256> {
        assert_ne!(denom, U256::default());
        let r = (self.as_u512() * num.as_u512()) / denom.as_u512();
        if r > U256::MAX.as_u512() {
            None
        } else {
            Some(r.as_u256())
        }
    }

    fn mul_div_ceil(self, num: U256, denom: U256) -> Option<U256> {
        assert_ne!(denom, U256::default());
        let r =
            (self.as_u512() * num.as_u512() + (denom - U256::from(1u8)).as_u512()) / denom.as_u512();
        if r > U256::MAX.as_u512() {
            None
        } else {
            Some(r.as_u256())
        }
    }
}