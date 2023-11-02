// #![deny(missing_debug_implementations, missing_copy_implementations)]
// #![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

extern crate core;

mod helper;
mod hosts;
pub use crate::hosts::{DecimalHost, FractionHost, IntegerHost, NaturalHost, OrdinalHost, PrimeHost};

wit_bindgen::generate!({
    world: "number-ffi",
    exports: {
        "vit:number/natural/natural": NaturalHost,
        "vit:number/integer": IntegerHost,
        "vit:number/ordinal": OrdinalHost,
        "vit:number/prime": PrimeHost,
        "vit:number/fraction": FractionHost,
        "vit:number/decimal": DecimalHost,
    },
});
