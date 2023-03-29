I'm 100% ignorant of Delphi and don't have a Windows machine at disposal. But anticipating a potential use case I would like to show that this Rust library can be used from it. 

Thus, I asked **@jrmaciel-zx** to:
1. Install Rust
1. Build the **calculations** Rust library as a dll via `cargo build --release --target=x86_64-pc-windows-gnu`
1. He set up the binding in **RustDelphi.dpr**. Note that the included code is 32-bit but 64-bit was tested as well.
    `function add(x, y: Float64): Float64; stdcall; external 'calculations.dll';`
1. Called `add(10.5, 20.45)`
1. It works - thanks **@jrmaciel-zx!**

The working example is included as a snapshot in time. To update it someone with Delphi on Windows will need to help.

Here is a screenshot of two inputs and the result.

<image src="./Delphi Output.png">
