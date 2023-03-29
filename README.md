Rust, the new hotness for fast but safe low level code. Linus is even putting it in the Linux kernel.

This repo demonstrates a simple calculation library being used by multiple client technologies. Calculations could be written once and used everywhere including in the browser. For good measure, unit tests could be created for each client reusing the same inputs.

| CLIENT | INTERFACE | Unit Tests |
|---|---|---|
| .Net | FFI via P/Invoke / DllImport | TODO |
| Delphi | FFI 64-bit on Windows (credit to **@jrmaciel-zx**) | TODO |
| Flutter | TODO | TODO |
| Java | JNI | TODO |
| Javascript Web | WASM | YES |
| Node.js | WASM | YES |
| Python | FFI | TODO |
| Ruby | FFI | TODO |
| Rust | Direct Dependency | TODO |
| Swift | TODO | TODO |
