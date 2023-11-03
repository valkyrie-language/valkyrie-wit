use num::BigUint;
use std::sync::LazyLock;
use wit_bindgen::rt::{Resource, WasmResource};

/// unsigned integer buffer
#[derive(Debug)]
#[repr(transparent)]
pub struct Natural {
    /// The owned value of natural
    ///
    /// offset = 0, size = 32
    pub own: BigUint,
}

impl Natural {
    /// Create a new natural number from the wasm pointer
    pub unsafe fn from_ptr<'i>(pointer: isize) -> &'i Resource<Natural> {
        &*(pointer as *const Resource<Natural>)
    }
    /// Convert the natural number into wasm pointer
    pub unsafe fn into_ptr(self: Resource<Natural>) -> isize {
        let ptr = &self as *const _;
        core::mem::forget(self);
        ptr as isize
    }
}

impl Natural {
    #[export_name = "number/Natural#[method]new-u32"]
    unsafe extern "C" fn new_u32(n: u32) -> isize {
        Resource::new(Natural { own: BigUint::from(n) }).into_ptr()
    }
    #[export_name = "number/Natural#[method]new-u64"]
    unsafe extern "C" fn new_u64(n: u64) -> isize {
        Resource::new(Natural { own: BigUint::from(n) }).into_ptr()
    }
    #[export_name = "number/Natural#[dtor]drop"]
    unsafe extern "C" fn destroy(this: isize) {
        let this = this as *mut Resource<Natural>;
        let _ = Box::from_raw(this);
    }
}

impl Natural {
    #[export_name = "number/Natural#[method]add-u32"]
    unsafe extern "C" fn add_u32(n: u32) -> isize {}
    #[export_name = "number/Natural#[method]add-u64"]
    unsafe extern "C" fn add_u64(n: u64) -> isize {}
    #[export_name = "number/Natural#[method]add-nat"]
    unsafe extern "C" fn add_nat(n: isize) -> isize {}
}
