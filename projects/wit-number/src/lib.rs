// #![deny(missing_debug_implementations, missing_copy_implementations)]
// #![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]


extern crate core;

mod hosts;
pub use crate::hosts::{NaturalHost, DecimalHost, IntegerHost};

wit_bindgen::generate!({
    world: "number-ffi",

    // For all exported worlds, interfaces, and resources, this specifies what
    // type they're corresponding to in this module. In this case the `MyHost`
    // struct defined below is going to define the exports of the `world`,
    // namely the `run` function.
    exports: {
        "vit:number/natural/natural": NaturalHost,
        "vit:number/decimal": DecimalHost,
        "vit:number/integer": IntegerHost,
    },
});
