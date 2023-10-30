#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]


use witgen::witgen;

// #[witgen]
// use other_crate::*;

/// Doc strings are supported!
#[witgen]
struct TestStruct {
    /// Even for fields!
    inner: String,
}

#[witgen]
enum TestEnum {
    Void,
    Number(u64),
    StringVariant(String),
}

#[witgen]
fn test(other: Vec<u8>, test_struct: TestStruct, other_enum: TestEnum) -> Result<(String, i64), String> {
    // The following code is not part of the generated `.wit` file.
    // You may add an example implementation or just satisfy the compiler with a `todo!()`.
    Ok((String::from("test"), 0i64))
}

#[witgen]
impl AResource {
    /// Can convert custom attributes to doc strings
    #[custom_attribute]
    pub fn foo(&self) {}
    /// Have special mutable attribute
    pub fn faa(&mut self) {}

    pub fn fee() {}
}