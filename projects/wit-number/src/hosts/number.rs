use num_bigint::BigUint;
use wit_bindgen::rt::{Resource, WasmResource};

/// unsigned integer buffer
#[derive(Debug)]
#[repr(transparent)]
pub struct Natural {
    raw: BigUint,
}

impl Natural {
    unsafe fn new_borrow<'i>(pointer: isize) -> &'i BigUint {}
}

impl Natural {
    #[export_name = "number/Natural#[method]new-u32"]
    unsafe extern "C" fn new_u32(n: u32) -> isize {}

    /// this: (&str, u64) -> option<unicode>
    #[export_name = "number/Natural#[method]get-char-nth"]
    unsafe extern "C" fn get_digits_nth(this: isize, nth: u64) -> u32 {}

    #[export_name = "number/Natural#[dtor]drop"]
    unsafe extern "C" fn destroy(this: isize) {}
}
