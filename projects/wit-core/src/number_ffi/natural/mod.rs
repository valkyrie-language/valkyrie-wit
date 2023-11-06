use crate::exports::v::core::number::GuestNatural;
use num::{BigUint, Zero};
use std::{ops::Add, sync::LazyLock};
use wit_bindgen::{
    rt::{Resource, WasmResource},
    RustResource,
};

/// unsigned integer buffer
#[derive(Debug)]
#[repr(transparent)]
pub struct Natural {
    /// The owned value of natural
    ///
    /// offset = 0, size = 32
    pub own: BigUint,
}

impl GuestNatural for Natural {
    fn new_u64(n: u64) -> crate::exports::v::core::number::Natural {
        todo!()
    }

    fn add_u64(&self, rhs: u64) -> crate::exports::v::core::number::Natural {
        todo!()
    }

    fn add_u64_inplace(&self, rhs: u64) {
        todo!()
    }

    fn add_nat(&self, rhs: crate::exports::v::core::number::Natural) -> crate::exports::v::core::number::Natural {
        todo!()
    }

    fn add_nat_inplace(&self, rhs: crate::exports::v::core::number::Natural) {
        todo!()
    }
}
