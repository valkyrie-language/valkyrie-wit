use anyhow::Result;
use wasmtime::Val;
use wit_test::ValkyrieVM;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn main1() -> Result<()> {
    let mut vm = ValkyrieVM::new();
    vm.load_binary("native", include_bytes!("../tests/wit_number.wasm"))?;
    vm.logging_linked();

    let str_ptr = vm.call_ffi_low("native", "text/Utf8Text#[method]new", &[])?;
    let char_ptr1 = vm.call_ffi_low("native", "text/Utf8Text#[method]get-char-nth", &[str_ptr[0].clone(), Val::I64(0)])?;
    let char_ptr2 = vm.call_ffi_low("native", "text/Utf8Text#[method]get-char-nth", &[str_ptr[0].clone(), Val::I64(1)])?;

    char_ptr1.iter().for_each(|v| println!("Res: {:?}", v));
    char_ptr2.iter().for_each(|v| println!("Res: {:?}", v));

    Ok(())
}
