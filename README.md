This repo demonstrates a simple write once, use everywhere calculation library being used by multiple client technologies, including in the browser via WASM/WebAssembly. For good measure, unit tests would be created for each client reusing the same inputs.

Why Rust? It is compatible with C, runs just or nearly as fast but is much safer. Linus is even putting it in the Linux kernel.

NOTE: All were created and tests on MacOS unless otherwise noted.

| CLIENT         | INTERFACE                                          | Unit Tests |
|----------------|----------------------------------------------------|---|
| .Net           | FFI via P/Invoke / DllImport                       | TODO |
| Delphi         | FFI 64-bit on Windows (credit to **@jrmaciel-zx**) | TODO |
| Dart / Flutter | FFI                                                | TODO |
| Java           | JNI                                                | TODO |
| Javascript Web | WASM                                               | YES |
| Node.js        | WASM                                               | YES |
| Python         | FFI                                                | TODO |
| Ruby           | FFI                                                | TODO |
| Rust           | Direct Dependency                                  | TODO |
| Swift          | TODO                                               | TODO |
