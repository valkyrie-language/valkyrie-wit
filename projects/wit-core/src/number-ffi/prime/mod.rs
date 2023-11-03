use super::*;
use crate::exports::vit::number::prime::{Guest, Ordinal, Prime};

impl Guest for PrimeHost {
    fn new(ordinal: Ordinal) -> Prime {
        todo!()
    }

    fn check_nat(n: OwnNatural) -> bool {
        todo!()
    }

    fn iter_next(self_: Prime) -> Prime {
        todo!()
    }

    fn iter_prev(self_: Prime) -> Prime {
        todo!()
    }

    fn around_prev(n: OwnNatural) -> Prime {
        todo!()
    }

    fn around_next(n: OwnNatural) -> Prime {
        todo!()
    }
}
