use crate::exports::vit::number::{
    decimal::Decimal,
    fraction::Fraction,
    integer::{Integer, Natural, Sign},
    natural::{GuestNatural, OwnNatural},
};
use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
use std::ops::Add;
use wit_bindgen::Resource;
mod impl_dec;
mod impl_frac;
mod impl_int;
mod impl_nat;
mod impl_ord;
mod impl_prime;
mod impl_sign;

pub struct NaturalHost {
    owned: BigUint,
}

pub struct IntegerHost {}

pub struct DecimalHost {}

pub struct FractionHost {}

pub struct OrdinalHost {}

pub struct PrimeHost {}
