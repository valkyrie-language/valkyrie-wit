use super::*;
use crate::exports::vit::number::integer::Guest;
use std::ops::Sub;

impl From<Integer> for BigInt {
    fn from(value: Integer) -> Self {
        BigInt::from_biguint(value.sign.into(), value.natural.owned.clone())
    }
}

impl From<BigInt> for Integer {
    fn from(value: BigInt) -> Self {
        Self { sign: value.sign().into(), natural: Resource::new(NaturalHost { owned: value.magnitude().clone() }) }
    }
}

impl Guest for IntegerHost {
    fn add_u32(self_: Integer, rhs: u32) -> Integer {
        BigInt::from(self_).add(&BigInt::from(rhs)).into()
    }

    fn add_u64(self_: Integer, rhs: u64) -> Integer {
        BigInt::from(self_).add(&BigInt::from(rhs)).into()
    }

    fn add_nat(self_: Integer, rhs: &Natural) -> Integer {
        todo!()
    }

    fn add_i32(self_: Integer, rhs: i32) -> Integer {
        BigInt::from(self_).add(&BigInt::from(rhs)).into()
    }

    fn add_i64(self_: Integer, rhs: i64) -> Integer {
        BigInt::from(self_).add(&BigInt::from(rhs)).into()
    }

    fn add_int(self_: Integer, rhs: Integer) -> Integer {
        todo!()
    }

    fn sub_u32(self_: Integer, rhs: u32) -> Integer {
        BigInt::from(self_).sub(&BigInt::from(rhs)).into()
    }

    fn sub_u64(self_: Integer, rhs: u64) -> Integer {
        BigInt::from(self_).sub(&BigInt::from(rhs)).into()
    }

    fn sub_nat(self_: Integer, rhs: &Natural) -> Integer {
        todo!()
    }

    fn sub_i32(self_: Integer, rhs: i32) -> Integer {
        BigInt::from(self_).sub(&BigInt::from(rhs)).into()
    }

    fn sub_i64(self_: Integer, rhs: i64) -> Integer {
        BigInt::from(self_).sub(&BigInt::from(rhs)).into()
    }

    fn sub_int(self_: Integer, rhs: Integer) -> Integer {
        todo!()
    }
}
