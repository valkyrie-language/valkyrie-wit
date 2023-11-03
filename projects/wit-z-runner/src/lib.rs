// #![deny(missing_debug_implementations, missing_copy_implementations)]
// #![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

use std::borrow::{Borrow, BorrowMut};
use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

struct MyState {
    message: String,
    wasi: WasiCtx,
}

#[test]
fn main() -> Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |state: &mut MyState| &mut state.wasi)?;

    let wasi = WasiCtxBuilder::new().inherit_stdio().inherit_args()?.build();
    let mut store = Store::new(&engine, MyState { message: format!("hello!"), wasi });

    // ...
    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(&engine, "tests/wit_text.wasm")?;
    linker.module(&mut store, "", &module)?;
    linker.get_default(&mut store, "")?.typed::<(), ()>(&store)?.call(&mut store, ())?;

    Ok(())
}
