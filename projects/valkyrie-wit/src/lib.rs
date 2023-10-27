#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "host",

    // For all exported worlds, interfaces, and resources, this specifies what
    // type they're corresponding to in this module. In this case the `MyHost`
    // struct defined below is going to define the exports of the `world`,
    // namely the `run` function.
    exports: {
        world: MyHost,
    },
});

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.
struct MyHost;

impl Guest for MyHost {
    fn run() {
        print("Hello, world!");
    }
}
