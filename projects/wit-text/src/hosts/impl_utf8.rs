use super::*;
use crate::exports::vit::text::utf8::{OwnUtf8Text, Utf8View};

impl Display for Unicode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.codepoint)
    }
}

impl GuestUtf8Text for UTF8Host {
    fn get_char_offset(&self, offset: u64) -> Option<Unicode> {
        todo!()
    }

    fn get_char_nth(&self, nth: u64) -> Option<Unicode> {
        self.ptr.chars().nth(nth as usize).map(|c| Unicode { codepoint: c })
    }

    fn get_view_offset_range(&self, start: u64, end: u64) -> Option<Utf8View> {
        todo!()
    }

    fn get_view_nth_range(&self, start: u64, end: u64) -> Option<Utf8View> {
        todo!()
    }
}
