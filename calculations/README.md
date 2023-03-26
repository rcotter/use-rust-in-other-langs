# C Libraries
1. Build `cargo build --release` from within this directory.
1. Outputs dynamic linking target **./target/release/libcalculations.dylib**


# WASM Library
1. Must have `wasm-pack` [installed](https://rustwasm.github.io/wasm-pack/installer) to output WASM for use by front end javascript or nodejs.
1. Must have `wasm-bindgen` [installed](cargo install -f wasm-bindgen-cli).
1. Generate: `wasm-pack build --target web`
1. `cargo build --target wasm32-unknown-unknown --release`
1. `wasm-bindgen target/wasm32-unknown-unknown/release/calculations.wasm --out-dir .`

