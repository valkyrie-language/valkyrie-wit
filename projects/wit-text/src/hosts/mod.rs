use num_traits::Zero;
use std::ops::Add;

mod impl_ascii;
mod impl_utf16;
mod impl_utf32;
mod impl_utf8;

pub struct AsciiHost {}

pub struct UTF8Host {}

pub struct UTF16Host {}

pub struct Utf32Host {}

pub struct NaturalHost {}

pub struct IntegerHost {}
