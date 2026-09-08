// Big integer types for liquidity math
// Using uint crate to construct U256 and U512 types

mod big_num_impl {
    use uint::construct_uint;

    construct_uint! {
        pub struct U256(4);
    }

    construct_uint! {
        pub struct U512(8);
    }
}

pub use big_num_impl::{U256, U512};

impl U256 {
    pub fn as_u512(self) -> U512 {
        U512([self.0[0], self.0[1], self.0[2], self.0[3], 0, 0, 0, 0])
    }
}

impl U512 {
    pub fn as_u256(self) -> U256 {
        U256([self.0[0], self.0[1], self.0[2], self.0[3]])
    }
}