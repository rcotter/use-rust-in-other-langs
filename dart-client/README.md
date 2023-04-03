1. Build updated Rust **calculations** library. 
2. Copy C lib header `calculations/targets/calculations.hpp` into the `lib` directory of this client.
3. Remove all lines except those for each function. See example as committed.
4. Use [ffigen](https://pub.dev/packages/ffigen) to generate dart bindings to the C library
   `dart run ffigen`
5. Run `dart run client.dart`