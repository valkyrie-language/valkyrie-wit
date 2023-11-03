use num::BigUint;
use wit_bindgen::rt::{Resource, WasmResource};

/// unsigned integer buffer
#[derive(Debug)]
#[repr(transparent)]
pub struct Natural {
    raw: BigUint,
}

impl Natural {
    unsafe fn new_borrow<'i>(pointer: isize) -> &'i BigUint {
        todo!()
    }
}

impl Natural {
    #[export_name = "number/Natural#[method]new-u32"]
    unsafe extern "C" fn new_u32(n: u32) -> isize {
        let nat = BigUint::from(n);
        let ptr = &nat as *const _;
        core::mem::forget(nat);
        ptr as isize
    }

    #[export_name = "number/Natural#[dtor]drop"]
    unsafe extern "C" fn destroy(this: isize) {
        let this = this as *mut BigUint;
        let _ = Box::from_raw(this);
    }
}
