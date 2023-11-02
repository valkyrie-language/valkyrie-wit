cargo build --release --target wasm32-wasi
wasm-tools component new ./target/wasm32-wasi/release/valkyrie_wit.wasm -o ./target/release/valkyrie-core.wasm --adapt ./wasi_snapshot_preview1.wasm
