Dynamic Linking: WORKS
Static Linking: NOT IMPLEMENTED

# Platform Setup
Verify and edit the library path in `class Calculations`. May have to add a "lib" prefix on non-MacOS:
  * **MacOS**: `System.loadLibrary("calculations");`
  * **Windows/Linux (TBD)**: `System.loadLibrary("libcalculations");`

# Makes Changes
1. Makes changes
1. `./publish.sh` to publish the JNI header to the Rust library.
1. Build the Rust library.

# Running
`./run.sh` to run.

It first copies the Rust lib into this Java project, generates the JNI interface header, compile Java classes and run the Java program.

# Future - Panama FFI
Java Panama FFI is being worked on to replace JNI.