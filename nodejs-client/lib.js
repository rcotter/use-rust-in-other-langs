var fs = require('fs');
const wasmBuffer = fs.readFileSync('../calculations/target/wasm32-unknown-unknown/release/calculations.wasm');

// TODO Upgrade to use WebAssembly.instantiateStreaming
async function load() {
    return (await WebAssembly.instantiate(wasmBuffer)).instance.exports;
}

module.exports = load;
