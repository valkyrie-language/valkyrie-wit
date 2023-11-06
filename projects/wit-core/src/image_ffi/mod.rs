use wit_bindgen::{Resource, RustResource, WasmResource};

// use num::BigUint;
// use std::{ops::Add, sync::LazyLock};
// use wit_bindgen::rt::{Resource, WasmResource};
//
/// unsigned integer buffer
#[derive(Debug)]
#[repr(transparent)]
pub struct RgbaImage {
    /// The owned value of natural
    ///
    /// offset = 0, size = 32
    pub own: image::RgbaImage,
}

impl RgbaImage {
    /// Create a new natural number-ffi from the wasm pointer
    pub unsafe fn from_ptr<'i>(pointer: isize) -> &'i Resource<RgbaImage> {
        &*(pointer as *const Resource<RgbaImage>)
    }
    /// Convert the natural number-ffi into wasm pointer
    pub unsafe fn into_ptr(this: Resource<RgbaImage>) -> isize {
        let ptr = &this as *const _;
        core::mem::forget(this);
        ptr as isize
    }
}

unsafe impl WasmResource for RgbaImage {
    unsafe fn drop(handle: u32) {
        let this = handle as *mut Resource<Self>;
        let _ = Box::from_raw(this);
    }
}

unsafe impl RustResource for RgbaImage {
    unsafe fn new(rep: usize) -> u32 {
        unreachable!()
    }

    unsafe fn rep(handle: u32) -> usize {
        let this = handle as *mut Resource<Self>;
        let new = this.clone();
        new as usize
    }
}

impl RgbaImage {
    #[export_name = "number-ffi/Image#[method]new"]
    unsafe extern "C" fn new(w: u32, h: u32) -> isize {
        RgbaImage::into_ptr(Resource::new(Self { own: image::RgbaImage::new(w, h) }))
    }
    // #[export_name = "number-ffi/Natural#[method]new-u64"]
    // unsafe extern "C" fn new_u64(n: u64) -> isize {
    //     Resource::new(RgbaImage { own: BigUint::from(n) }).into_ptr()
    // }
    #[export_name = "number-ffi/Image#[dtor]drop"]
    unsafe extern "C" fn destroy(this: isize) {
        let this = this as *mut Resource<Self>;
        let _ = Box::from_raw(this);
    }
}
