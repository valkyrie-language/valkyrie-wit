#[test]
fn ready() {
    println!("it works!")
}

use std::borrow::{Borrow, BorrowMut};
use wasmtime::{HostAbi, *};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};
use wit_test::ValkyrieRuntime;

#[test]
fn main1() -> Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |state: &mut ValkyrieRuntime| &mut state.wasi)?;

    let wasi = WasiCtxBuilder::new().inherit_stdio().inherit_args()?.build();
    let mut store = Store::new(&engine, ValkyrieRuntime { message: format!("hello!"), wasi });

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(&engine, "tests/wit_number.wasm")?;
    for x in module.exports() {
        // println!("Out: {:?}", x);
    }
    linker.module(&mut store, "vit:number", &module)?;
    for item in linker.iter(&mut store) {
        // println!("Ext: {:?}", item);
    }

    let ext = linker.get(&mut store, "", "vit:number/natural#[constructor]natural").expect("missing");
    let ext = match ext {
        Extern::Func(v) => v,
        _ => {
            unreachable!()
        }
    };
    // let ext = Extern::Func(Func::wrap(store, || {}));
    let input = vec![Val::F32(0)];
    let mut output = vec![Val::F32(0)];
    let out = ext.call(&mut store, &input, &mut output);
    println!("Res: {:?}", out);
    println!("Res: {:?}", output);
    Ok(())
}
