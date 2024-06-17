use std::path::Path;
use valkyrie_ffi::ValkyrieFFI;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn load_deps() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let resolved = ValkyrieFFI::new_deps(here.join("tests/wasi-cloud-core/wit")).unwrap();
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    resolved.generate(here).unwrap();
}

#[test]
fn load_wasm() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let file = std::fs::read(here.join("host.wasm")).unwrap();
    let resolved = ValkyrieFFI::new_wasm(&file, "homestar").unwrap();
    resolved.generate(here).unwrap();
}
