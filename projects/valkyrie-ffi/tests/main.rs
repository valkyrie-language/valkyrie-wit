use std::path::Path;
use valkyrie_wit::ValkyrieFFI;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn load_all() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let resolved = ValkyrieFFI::new(here.join("tests/wasi-cloud-core/wit")).unwrap();
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    resolved.generate(here).unwrap();
}
