This repo demonstrates a simple write once, use everywhere calculation library being used by multiple client technologies, including in the browser via WASM/WebAssembly. For good measure, unit tests would be created for each client reusing the same inputs.

Why Rust? It is compatible with C, runs just or nearly as fast but is much safer. Linus is even putting it in the Linux kernel. Checkout some [metrics](https://github.com/dyu/ffi-overhead) and [caveats](https://news.ycombinator.com/item?id=31376679).

NOTE: All were created and tests on MacOS unless otherwise noted.

| CLIENT         | INTERFACE                                          | Unit Tests |
|----------------|----------------------------------------------------|:---:|
| .Net           | FFI via P/Invoke / DllImport                       |  |
| Delphi         | FFI 64-bit on Windows (credit to **@jrmaciel-zx**) |  |
| Dart / Flutter | FFI via FFIGEN & FFI.                              |  |
| Java           | JNI                                                |  |
| Javascript Web | WASM / WebAssembly                                 | ✅ |
| Node.js        | WASM / WebAssembly                                 | ✅ |
| Python         | FFI                                                |  |
| Ruby           | FFI                                                |  |
| Rust           | Direct Dependency                                  |  |
| Swift          | Won't Do - Lots of Examples Available              |  |
