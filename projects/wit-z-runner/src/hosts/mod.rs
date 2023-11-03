use crate::exports::vit::text::{
    ascii::{Ascii, GuestAsciiText},
    utf8::{GuestUtf8Text, Unicode},
};
use num_traits::Zero;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Add,
};

mod impl_ascii;
mod impl_utf16;
mod impl_utf32;
mod impl_utf8;

pub struct AsciiHost {
    ptr: String,
}

pub struct UTF8Host {
    ptr: String,
}

pub struct UTF16Host {}

pub struct Utf32Host {}

pub struct NaturalHost {}

pub struct IntegerHost {}
