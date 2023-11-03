// #![deny(missing_debug_implementations, missing_copy_implementations)]
// #![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

use std::{
    borrow::{Borrow, BorrowMut},
    collections::BTreeMap,
};
use wasmtime::{HostAbi, *};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

pub struct ValkyrieVM {
    engine: Engine,
    linker: Linker<ValkyrieContext>,
    store: Store<ValkyrieContext>,
}

pub struct ValkyrieContext {
    pub message: String,
    pub wasi: WasiCtx,
}

pub struct Utf8Text {
    _pointer: u32,
    length: u64,
}

pub struct ExternalCall<'vm> {
    name: &'vm str,
}

impl<'i> ExternalCall<'i> {
    pub fn new(name: &'i str) -> Self {
        Self { name }
    }
    fn prepare(&self, store: &mut Store<ValkyrieContext>, linker: &mut Linker<ValkyrieContext>) -> Option<Func> {
        let f_new = linker.get(store, "native", self.name)?;
        match f_new {
            Extern::Func(v) => Some(v),
            _ => None,
        }
    }
}

impl ValkyrieVM {
    pub fn new() -> Self {
        let engine = Engine::default();
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
    pub fn log_linked(&mut self) {
        let width = self.linker.iter(&mut self.store).map(|(module, name, _)| module.len() + name.len() + 2).max().unwrap_or(0);
        for (module, name, function) in self.linker.iter(&mut self.store) {
            print!(
                "{module}:{name}{padding}=> ",
                module = module,
                name = name,
                padding = " ".repeat(width - module.len() - name.len() - 2)
            );
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

    fn call_ffi_low(&mut self, module: &str, name: &str, input: &[Val]) -> Result<Vec<Val>> {
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

pub enum ValkyrieValue {
    Nat32(u32),
    Nat64(u64),
    Int32(i32),
    Int64(i64),
    Unicode(char),
}

#[test]
fn main1() -> Result<()> {
    let mut vm = ValkyrieVM::new();
    vm.load_binary("native", include_bytes!("../tests/wit_number.wasm"))?;
    vm.log_linked();

    let str_ptr = vm.call_ffi_low("native", "v:number/integer#[method]new", &[])?;
    let char_ptr1 = vm.call_ffi_low("native", "v:number/integer#[method]add-u32", &[str_ptr[0].clone(), Val::I64(0)])?;
    let char_ptr2 = vm.call_ffi_low("native", "v:number/integer#[method]add-u32", &[str_ptr[0].clone(), Val::I64(1)])?;

    char_ptr1.iter().for_each(|v| println!("Res: {:?}", v));
    char_ptr2.iter().for_each(|v| println!("Res: {:?}", v));

    Ok(())
}
