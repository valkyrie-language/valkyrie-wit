use wasmtime::{
    component::{Component, Linker},
    *,
};

#[test]
fn main() -> wasmtime::Result<()> {
    let engine = Engine::default();
    // my_wasm.wasm was compiled by the guest above
    let module = Component::from_file(&engine, "test/my_wasm.wasm")?;

    let mut linker = Linker::new(&engine);
    // linker.func_wrap("host", "print", |caller: Caller<'_, u32>, param: i32| {
    //     println!("{}", param);
    //     println!("my host state is: {}", caller.data());
    // })?;

    let mut store = Store::new(&engine, 0);
    let instance = linker.instantiate(&mut store, &module)?;
    let hello = instance.get_typed_func::<(), ()>(&mut store, "run")?;
    hello.call(&mut store, ())?;

    Ok(())
}
