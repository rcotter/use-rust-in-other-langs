var fs = require('fs');
const wasmBuffer = fs.readFileSync('../calculations/target/wasm32-unknown-unknown/release/calculations.wasm');

WebAssembly
  .instantiate(wasmBuffer)
  .then(wasmModule => {
    const { add, distance_between_two_points_on_the_earth } = wasmModule.instance.exports;
    distance_between_two_points_on_the_earth();
    const sum = add(5, 6);
    console.log(`5 + 6 = ${sum}`);
  });
