use super::*;

impl Display for Ascii {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.codepoint as char)
    }
}

impl GuestAsciiText for AsciiHost {
    fn get_char_offset(&self, offset: u64) -> Option<Ascii> {
        let c = self.ptr.bytes().nth(offset as usize)?;
        Some(Ascii { codepoint: c })
    }
}
