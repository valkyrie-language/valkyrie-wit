use super::*;
use crate::exports::vit::number::ordinal::{Guest, Ordinal};

impl Guest for OrdinalHost {
    fn new(integer: Integer) -> Option<Ordinal> {
        if integer.natural.owned.is_zero() {
            return None;
        }
        Some(Ordinal { integer })
    }

    fn uncheck_new(integer: Integer) -> Ordinal {
        Ordinal { integer }
    }
}
