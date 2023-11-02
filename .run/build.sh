cargo build --target wasm32-wasi
wasm-tools component new ./target/wasm32-wasi/debug/wit-number.wasm -o ./target/debug/vit-number.wasm --adapt ./wasi_snapshot_preview1.wasm
