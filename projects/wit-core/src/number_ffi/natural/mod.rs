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

impl Natural {
    /// Create a new natural number-ffi from the wasm pointer
    pub unsafe fn from_ptr<'i>(pointer: isize) -> &'i Resource<Natural> {
        &*(pointer as *const Resource<Natural>)
    }
    /// Convert the natural number-ffi into wasm pointer
    pub unsafe fn into_ptr(this: Resource<Natural>) -> u32 {
        let ptr = &this as *const _;
        core::mem::forget(this);
        ptr as u32
    }
}

unsafe impl WasmResource for Natural {
    unsafe fn drop(handle: u32) {
        let this = handle as *mut Resource<Natural>;
        let _ = Box::from_raw(this);
    }
}
unsafe impl RustResource for Natural {
    unsafe fn new(_: usize) -> u32 {
        Self::into_ptr(Resource::new(Natural { own: BigUint::zero() }))
    }

    unsafe fn rep(handle: u32) -> usize {
        todo!()
    }
}

impl Natural {
    #[export_name = "number-ffi/Natural#[method]new-u32"]
    unsafe extern "C" fn new_u32(n: u32) -> u32 {
        Self::into_ptr(Resource::new(Natural { own: BigUint::from(n) }))
    }
    #[export_name = "number-ffi/Natural#[method]new-u64"]
    unsafe extern "C" fn new_u64(n: u64) -> u32 {
        Self::into_ptr(Resource::new(Natural { own: BigUint::from(n) }))
    }
    #[export_name = "number-ffi/Natural#[dtor]drop"]
    unsafe extern "C" fn destroy(this: isize) {
        let this = this as *mut Resource<Natural>;
        let _ = Box::from_raw(this);
    }
}

impl Natural {
    #[export_name = "number-ffi/Natural#[method]add-u32"]
    unsafe extern "C" fn add_u32(this: isize, rhs: u32) -> u32 {
        let n = Self::from_ptr(this);
        let out = n.own.clone().add(&BigUint::from(rhs));
        Self::into_ptr(Resource::new(Natural { own: out }))
    }
    #[export_name = "number-ffi/Natural#[method]add-u64"]
    unsafe extern "C" fn add_u64(this: isize, rhs: u64) -> u32 {
        let n = Self::from_ptr(this);
        let out = n.own.clone().add(&BigUint::from(rhs));
        Self::into_ptr(Resource::new(Natural { own: out }))
    }
    #[export_name = "number-ffi/Natural#[method]add-nat"]
    unsafe extern "C" fn add_nat(this: isize, rhs: isize) -> u32 {
        let n = Self::from_ptr(this);
        let rhs = Self::from_ptr(rhs);
        let out = n.own.clone().add(&rhs.own);
        Self::into_ptr(Resource::new(Natural { own: out }))
    }
}
