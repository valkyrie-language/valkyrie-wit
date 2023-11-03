// use crate::exports::vit::number-ffi::{
//     decimal::Decimal,
//     fraction::Fraction,
//     integer::{Integer, Natural, Sign},
//     natural::{GuestNatural, OwnNatural},
// };
// use num_bigint::{BigInt, BigUint};
// use num_traits::Zero;
// use std::ops::Add;
// use wit_bindgen::Resource;
//
pub mod decimal;
pub mod fraction;
pub mod integer;
pub mod natural;
pub mod ordinal;
pub mod prime;
pub mod sign;
// pub struct NaturalHost {
//     owned: BigUint,
// }
//
// pub struct IntegerHost {}
//
// pub struct DecimalHost {}
//
// pub struct FractionHost {}
//
// pub struct OrdinalHost {}
//
// pub struct PrimeHost {}
