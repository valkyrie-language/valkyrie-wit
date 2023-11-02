use crate::exports::vit::number::{
    decimal,
    decimal::Decimal,
    integer::{Integer, Natural, Sign},
    natural::{GuestNatural, OwnNatural},
};
use num_bigint::{BigInt, BigUint};
use std::ops::Add;
use wit_bindgen::Resource;

mod impl_int;
mod impl_nat;
mod impl_sign;

pub struct NaturalHost {
    owned: BigUint,
}

pub struct IntegerHost {}

pub struct DecimalHost {}

impl decimal::Guest for DecimalHost {
    fn add_f32(rhs: f32) -> Decimal {
        todo!()
    }

    fn add_f64(rhs: f64) -> Decimal {
        todo!()
    }
}
