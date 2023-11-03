use num::BigUint;
use std::{ops::Add, sync::LazyLock};
use wit_bindgen::rt::{Resource, WasmResource};

/// unsigned integer buffer
#[derive(Debug)]
#[repr(transparent)]
pub struct RgbaImage {
    /// The owned value of natural
    ///
    /// offset = 0, size = 32
    pub own: RgbaImage,
}

impl RgbaImage {
    /// Create a new natural number-ffi from the wasm pointer
    pub unsafe fn from_ptr<'i>(pointer: isize) -> &'i Resource<RgbaImage> {
        &*(pointer as *const Resource<RgbaImage>)
    }
    /// Convert the natural number-ffi into wasm pointer
    pub unsafe fn into_ptr(self: Resource<RgbaImage>) -> isize {
        let ptr = &self as *const _;
        core::mem::forget(self);
        ptr as isize
    }
}

impl RgbaImage {
    #[export_name = "number-ffi/Natural#[method]new-u32"]
    unsafe extern "C" fn new_u32(n: u32) -> isize {
        Resource::new(RgbaImage { own: BigUint::from(n) }).into_ptr()
    }
    #[export_name = "number-ffi/Natural#[method]new-u64"]
    unsafe extern "C" fn new_u64(n: u64) -> isize {
        Resource::new(RgbaImage { own: BigUint::from(n) }).into_ptr()
    }
    #[export_name = "number-ffi/Natural#[dtor]drop"]
    unsafe extern "C" fn destroy(this: isize) {
        let this = this as *mut Resource<RgbaImage>;
        let _ = Box::from_raw(this);
    }
}

impl RgbaImage {
    #[export_name = "number-ffi/Natural#[method]add-u32"]
    unsafe extern "C" fn add_u32(this: isize, rhs: u32) -> isize {
        let n = Self::from_ptr(this);
        let out = n.own.add(&BigUint::from(rhs));
        Resource::new(RgbaImage { own: out }).into_ptr()
    }
    #[export_name = "number-ffi/Natural#[method]add-u64"]
    unsafe extern "C" fn add_u64(this: isize, rhs: u64) -> isize {
        let n = Self::from_ptr(this);
        let out = n.own.add(&BigUint::from(rhs));
        Resource::new(RgbaImage { own: out }).into_ptr()
    }
    #[export_name = "number-ffi/Natural#[method]add-nat"]
    unsafe extern "C" fn add_nat(this: isize, rhs: isize) -> isize {
        let n = Self::from_ptr(this);
        let rhs = Self::from_ptr(rhs);
        let out = n.own.add(&rhs.own);
        Resource::new(RgbaImage { own: out }).into_ptr()
    }
}
