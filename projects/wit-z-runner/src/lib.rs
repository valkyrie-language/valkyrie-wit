// #![deny(missing_debug_implementations, missing_copy_implementations)]
// #![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

use std::borrow::{Borrow, BorrowMut};
use wasmtime::{HostAbi, *};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

pub struct ValkyrieRuntime {
    pub message: String,
    pub wasi: WasiCtx,
}

pub struct Utf8Text {
    _pointer: u32,
    length: u64,
}

impl ValkyrieRuntime {
    pub fn load_function(&mut self, name: &str) -> Func {
        let mut store = self.store();
        let mut linker = self.linker();
        let module = Module::from_file(&self.engine(), "tests/wit_number.wasm").unwrap();
        linker.module(&mut store, "v:text", &module).unwrap();
        let f_new = linker.get(&mut store, "native", name).expect("missing");
        let f_new = match f_new {
            Extern::Func(v) => v,
            _ => {
                unreachable!()
            }
        };
        f_new
    }
}

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
        println!("Out: {:?}", x);
    }
    linker.module(&mut store, "v:text", &module)?;

    for item in linker.iter(&mut store) {
        // println!("Ext: {:?}", item);
    }

    let f_new = linker.get(&mut store, "native", "v:text/Utf8Text#[method]new").expect("missing");
    let f_new = match f_new {
        Extern::Func(v) => v,
        _ => {
            unreachable!()
        }
    };
    let f_get_char = linker.get(&mut store, "native", "v:text/Utf8Text#[method]get-char-nth").expect("missing");
    let f_get_char = match f_get_char {
        Extern::Func(v) => v,
        _ => {
            unreachable!()
        }
    };

    // let ext = Extern::Func(Func::wrap(store, || {}));
    let input = vec![];
    let mut str_ptr = vec![Val::I32(0)];
    let mut char_ptr = vec![Val::I32(0)];
    let out = f_new.call(&mut store, &input, &mut str_ptr);
    println!("Res: {:?}", out);
    println!("Res: {:?}", str_ptr);
    str_ptr.push(Val::I64(0));
    let out = f_get_char.call(&mut store, &str_ptr, &mut char_ptr);
    println!("Res: {:?}", out);
    match char_ptr.first() {
        Some(Val::I32(c)) => match char::from_u32(*c as u32) {
            None => {}
            Some(s) => {
                println!("Res: {:?}", s);
            }
        },
        _ => {}
    }
    let out = f_get_char.call(&mut store, &str_ptr, &mut char_ptr);
    str_ptr[1] = Val::I64(1);
    println!("Res: {:?}", out);
    match char_ptr.first() {
        Some(Val::I32(c)) => match char::from_u32(*c as u32) {
            None => {}
            Some(s) => {
                println!("Res: {:?}", s);
            }
        },
        _ => {}
    }

    Ok(())
}
