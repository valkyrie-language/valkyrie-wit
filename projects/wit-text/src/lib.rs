// #![deny(missing_debug_implementations, missing_copy_implementations)]
// #![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

extern crate core;

mod helper;
mod hosts;
pub use crate::hosts::{AsciiHost, IntegerHost, NaturalHost, UTF16Host, UTF8Host, Utf32Host};

wit_bindgen::generate!({
    world: "text-ffi",
    exports: {
        "vit:text/ascii/ascii-text": UTF8Host,
        "vit:text/utf8/utf8-text": UTF8Host,
    },
});
