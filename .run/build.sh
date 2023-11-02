cargo build --target wasm32-wasi
wasm-tools component new ./target/wasm32-wasi/debug/wit_number.wasm -o ./target/wasm32-wasi/debug/vit_number.wasm --adapt ./wasi_snapshot_preview1.wasm
