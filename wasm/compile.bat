cargo +nightly build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/wasm.wasm -o wasm.wasm

PAUSE