use anyhow::Result;
use wasmtime::{component::*, Config, Engine, Store};
use wit_test::ValkyrieVM;

#[test]
fn ready() {
    println!("it works!")
}

const WASM: &[u8] = include_bytes!("../../../target/wasm32-wasi/debug/component.wasm");

#[test]
fn main1() -> Result<()> {
    // Create a new engine for instantiating a component.
    let engine = Engine::new(&Config::new().wasm_component_model(true)).expect("fail to initialize ValkyrieVM");

    // Create a store for managing WASM data and any custom user-defined state.
    let mut store = Store::new(&engine, ());

    // Parse the component bytes and load its imports and exports.
    let component = Component::new(&engine, WASM).unwrap();
    // Create a linker that will be used to resolve the component's imports, if any.
    let linker = Linker::new(&engine);
    // Create an instance of the component using the linker.
    let instance = linker.instantiate(&mut store, &component).unwrap();

    // Get the interface that the interface exports.
    let mut export = instance.exports(&mut store);
    for x in store.root().modules() {
        println!("Export: {:#?}", x);
    }
    let mut interface = export.instance(&"test:guest/foo").unwrap();
    // Get the function for selecting a list element.
    let select_nth = interface.func("select-nth").unwrap();

    // Create an example list to test upon.
    // let example = ["a", "b", "c"].iter().map(ToString::to_string).collect::<Vec<_>>();

    // println!("Calling select-nth({example:?}, 1) == {}", select_nth.call(&mut store, (example.clone(), 1)).unwrap());

    Ok(())
}
