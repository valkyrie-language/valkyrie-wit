// #![deny(missing_debug_implementations, missing_copy_implementations)]
// #![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

use itertools::Itertools;
use std::{
    borrow::{Borrow, BorrowMut},
    collections::BTreeMap,
};
use wasmtime::{HostAbi, *};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

mod values;
pub struct ValkyrieVM {
    engine: Engine,
    linker: Linker<ValkyrieContext>,
    store: Store<ValkyrieContext>,
}

pub struct ValkyrieContext {
    pub message: String,
    pub wasi: WasiCtx,
}

impl ValkyrieVM {
    pub fn new() -> Self {
        let engine = Engine::new(&Config::new().wasm_component_model(true)).expect("fail to initialize ValkyrieVM");
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |state: &mut ValkyrieContext| &mut state.wasi).unwrap();
        let wasi = WasiCtxBuilder::new().inherit_stdio().inherit_args().unwrap().build();
        let store = Store::new(&engine, ValkyrieContext { message: "".to_string(), wasi });
        Self { engine, linker, store }
    }

    pub fn load_binary(&mut self, module: &str, binary: &[u8]) -> Result<()> {
        let load = Module::from_binary(&self.engine, binary)?;
        self.linker.module(&mut self.store, module, &load)?;
        Ok(())
    }
    pub fn logging_linked(&mut self) {
        let mut pad1 = 0;
        let mut pad2 = 0;
        for (module, name, _) in self.linker.iter(&mut self.store) {
            pad1 = std::cmp::max(pad1, module.len());
            pad2 = std::cmp::max(pad2, name.len());
        }
        for (module, name, function) in self.linker.iter(&mut self.store).sorted_by_key(|v| (v.0, v.1)) {
            print!("{module:<pad1$}  {name:<pad2$} => ", pad1 = pad1, pad2 = pad2);
            match function {
                Extern::Func(v) => println!("{v:?}"),
                Extern::Global(v) => println!("{v:?}"),
                Extern::Table(v) => println!("{v:?}"),
                Extern::Memory(v) => println!("{v:?}"),
                Extern::SharedMemory(v) => println!("{v:?}"),
            }
        }
    }
    pub fn load_function(&mut self, module: &str, name: &str) -> Option<Func> {
        let external = self.linker.get(&mut self.store, module, name)?;
        match external {
            Extern::Func(v) => Some(v),
            _ => None,
        }
    }

    pub fn call_ffi_low(&mut self, module: &str, name: &str, input: &[Val]) -> Result<Vec<Val>> {
        let f = match self.load_function(module, name) {
            Some(s) => s,
            None => Err(anyhow::anyhow!("Function {name} in {module} not found"))?,
        };
        let len = f.ty(&self.store).results().count();
        let mut output = vec![Val::I32(0); len];
        f.call(&mut self.store, input, &mut output)?;
        Ok(output)
    }
}
