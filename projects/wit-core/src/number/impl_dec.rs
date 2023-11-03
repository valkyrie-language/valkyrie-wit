use super::*;
use crate::exports::vit::number::decimal::Guest;

impl Guest for DecimalHost {
    fn add_f32(self_: Decimal, rhs: f32) -> Decimal {
        todo!()
    }

    fn add_f64(self_: Decimal, rhs: f64) -> Decimal {
        todo!()
    }
}
