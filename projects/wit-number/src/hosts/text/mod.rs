use wit_bindgen::rt::{Resource, WasmResource};

/// unsigned integer buffer
#[derive(Debug)]
#[repr(transparent)]
pub struct Utf8Text {
    raw: String,
}

impl Utf8Text {
    unsafe fn new_borrow<'i>(pointer: isize) -> &'i str {
        let this = pointer as *const u8;
        let len_ptr = this.sub(4);
        let len = *(len_ptr as *const u32) as usize;
        let this = std::slice::from_raw_parts(this, len);
        std::str::from_utf8_unchecked(this)
    }
}

impl Utf8Text {
    #[export_name = "text/Utf8Text#[method]new"]
    unsafe extern "C" fn new() -> isize {
        let rust_string = "Hello, world!".to_string();
        let ptr = rust_string.as_ptr();
        std::mem::forget(rust_string);
        ptr as isize
    }

    /// this: (&str, u64) -> option<unicode>
    #[export_name = "text/Utf8Text#[method]get-char-nth"]
    unsafe extern "C" fn get_char_nth(this: isize, nth: u64) -> u32 {
        let mut out = '\0';
        if nth != 0 {
            let this = Self::new_borrow(this);
            match this.chars().nth(nth as usize) {
                Some(o) => out = o,
                None => {}
            }
        }
        out as u32
    }

    #[export_name = "text/Utf8Text#[dtor]get-char-nth"]
    unsafe extern "C" fn destroy(this: isize) {
        let this = this as *mut String;
        let _ = Box::from_raw(this);
    }
}
