use super::*;

/// unsigned integer buffer
#[repr(align(4))]
#[repr(C, align(4))]
pub struct Integer {
    // offset = 0, size = 8
    sign: Sign,
    // offset = 8, size = 32
    raw: Resource<Natural>,
}

#[derive(Debug)]
#[repr(i8)]
pub enum Sign {
    UnSign = 0,
    Positive = 1,
    Negative = -1,
}

impl Integer {
    /// Create a new [Integer] from wasm pointer
    pub unsafe fn from_ptr<'i>(address: isize) -> &'i Integer {
        &*(address as *const Self)
    }
    /// Create a wasm pointer from [Integer]
    pub unsafe fn into_ptr(self) -> isize {
        let ptr = &self as *const _;
        core::mem::forget(self);
        ptr as isize
    }
}

impl Integer {
    #[export_name = "number-ffi/Integer#[method]new-u32"]
    unsafe extern "C" fn new_u32(n: u32) -> isize {
        unreachable!()
    }

    #[export_name = "number-ffi/Integer#[dtor]drop"]
    unsafe extern "C" fn destroy(this: isize) {
        let this = this as *mut BigUint;
        let _ = Box::from_raw(this);
    }
}
