use crate::exports::vit::native::{decimal, integer};
use crate::exports::vit::native::decimal::Decimal;
use crate::exports::vit::native::integer::{Integer, Natural};
use crate::exports::vit::native::natural::{GuestNatural, OwnNatural};

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

    fn add_u64(self_: Integer, rhs: u64) -> Integer {
        todo!()
    }

    fn add_nat(self_: Integer, rhs: &Natural) -> Integer {
        todo!()
    }

    fn add_i32(self_: Integer, rhs: i32) -> Integer {
        todo!()
    }

    fn add_i64(self_: Integer, rhs: i64) -> Integer {
        todo!()
    }

    fn add_int(self_: Integer, rhs: Integer) -> Integer {
        todo!()
    }
}

pub struct DecimalHost;

impl decimal::Guest for DecimalHost {
    fn add_f32(rhs: f32) -> Decimal {
        todo!()
    }

    fn add_f64(rhs: f64) -> Decimal {
        todo!()
    }
}
