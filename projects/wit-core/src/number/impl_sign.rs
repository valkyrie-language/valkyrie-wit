use super::*;
use num_bigint::Sign::{Minus, NoSign, Plus};
use Sign::{Negative, Positive, UnSign};

impl From<Sign> for num_bigint::Sign {
    fn from(value: Sign) -> Self {
        match value {
            UnSign => NoSign,
            Positive => Plus,
            Negative => Minus,
        }
    }
}

impl From<num_bigint::Sign> for Sign {
    fn from(value: num_bigint::Sign) -> Self {
        match value {
            NoSign => UnSign,
            Plus => Positive,
            Minus => Negative,
        }
    }
}
