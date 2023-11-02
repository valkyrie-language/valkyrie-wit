cargo build --target wasm32-wasi
wasm-tools component new ./target/wasm32-wasi/debug/valkyrie_wit.wasm -o ./target/debug/valkyrie-core.wasm --adapt ./wasi_snapshot_preview1.wasm
