use anyhow::Result;
use wasmtime::{component::*, Config, Engine, Store};
use wit_test::ValkyrieVM;

#[test]
fn ready() {
    println!("it works!")
}

const WASM: &[u8] = include_bytes!("../../../target/wasm32-wasi/debug/valkyrie-wit.wasm");

#[test]
fn main1() -> Result<()> {
    // Create a new engine for instantiating a component.
    let engine = Engine::new(&Config::new().wasm_component_model(true)).expect("fail to initialize ValkyrieVM");
    let mut store = Store::new(&engine, ());
    let component = Component::new(&engine, WASM).unwrap();
    let linker = Linker::new(&engine);

    let instance = linker.instantiate(&mut store, &component).unwrap();

    let mut select_nth = instance.get_func(&mut store, "test:guest/foo").unwrap();

    let mut out = vec![Val::Bool(false)];
    select_nth.call(&mut store, &[Val::Bool(true)], &mut out).unwrap();
    Ok(())
}
