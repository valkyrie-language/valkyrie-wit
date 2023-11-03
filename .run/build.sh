cargo build --target wasm32-wasi
wasm-tools component new ./target/wasm32-wasi/debug/valkyrie_ffi.wasm -o ./target/wasm32-wasi/debug/valkyrie-wit.wasm --adapt ./wasi_snapshot_preview1.wasm


