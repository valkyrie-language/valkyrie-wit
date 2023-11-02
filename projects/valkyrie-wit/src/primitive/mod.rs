use crate::exports::vit::core::{decimal, integer};
use crate::exports::vit::core::decimal::Decimal;
use crate::exports::vit::core::integer::Integer;
use crate::exports::vit::core::natural::{GuestNatural, OwnNatural};


pub struct NaturalHost;



impl GuestNatural for NaturalHost {
    fn new(init: Vec<u32>) -> Self {
        todo!()
    }

    fn add_u32(&self, rhs: u32) -> OwnNatural {
        todo!()
    }

    fn add_u64(&self, rhs: u64) -> OwnNatural {
        todo!()
    }

    fn add_nat(&self, rhs: &NaturalHost) -> OwnNatural {
        todo!()
    }
}


pub struct IntegerHost;
impl integer::Guest for IntegerHost {
    fn add_u32(self_: Integer, rhs: u32) -> Integer {
        todo!()
    }
}

pub struct DecimalHost;

impl decimal::Guest for IntegerHost {
    fn add_u32(rhs: u32) -> Decimal {
        todo!()
    }

    fn add_u64(rhs: u64) -> Decimal {
        todo!()
    }
}
