#![feature(lazy_cell)]
// #![deny(missing_debug_implementations, missing_copy_implementations)]
// #![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![allow(clippy::wrong_self_convention)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

include!("exports.rs");
mod helper;
pub mod image_ffi;
mod number_ffi;
pub mod text_ffi;

pub use crate::number_ffi::{integer::numberHost, natural::NaturalHost};
