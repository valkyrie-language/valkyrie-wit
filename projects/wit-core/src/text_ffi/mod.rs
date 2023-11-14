use crate::exports::v::core::{
    number::Integer,
    text::{Ascii, Guest, Unicode},
};

/// unsigned integer buffer
#[derive(Debug)]
pub struct Utf8Text {
    own: String,
}

pub struct TextFFI {}

impl Guest for TextFFI {
    fn ascii_add_u8(this: Ascii, rhs: u8) -> Ascii {
        Ascii { codepoint: this.codepoint.saturating_add(rhs) }
    }

    fn unicode_add_u32(this: Unicode, rhs: u32) -> Unicode {
        Unicode { codepoint: this.codepoint.saturating_add(rhs) }
    }
}
