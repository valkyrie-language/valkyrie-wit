use super::*;

/// unsigned integer buffer

pub struct NumberFFI {}

impl Guest for NumberFFI {
    fn nat_new_u64(n: u64) -> Natural {
        todo!()
    }

    fn nat_add_u64(this: Natural, rhs: u64) -> Natural {
        todo!()
    }

    fn nat_add_u64_inplace(this: Natural, rhs: u64) {
        todo!()
    }

    fn nat_add_nat(this: Natural, rhs: Natural) -> Natural {
        todo!()
    }

    fn nat_add_nat_inplace(this: Natural, rhs: Natural) {
        todo!()
    }

    fn int_add_u64(this: Integer, rhs: u64) -> Integer {
        todo!()
    }

    fn int_add_nat(this: Integer, rhs: Natural) -> Integer {
        todo!()
    }

    fn int_add_int(this: Integer, rhs: Integer) -> Integer {
        todo!()
    }
}
