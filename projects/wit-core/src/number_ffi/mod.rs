use crate::number_ffi::natural::Natural;
use num::{BigInt, BigUint};
use wit_bindgen::rt::{Resource, WasmResource};
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
