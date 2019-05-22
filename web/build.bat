cargo +nightly build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/web.wasm --out-dir ./target/wasm32-unknown-unknown