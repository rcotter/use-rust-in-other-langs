let wasm;

WebAssembly.instantiateStreaming(fetch("calculations_bg.wasm")).then(
    obj => wasm = obj.instance.exports
  );