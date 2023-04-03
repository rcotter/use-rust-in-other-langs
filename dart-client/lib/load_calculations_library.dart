import 'dart:ffi';
import 'dart:io';
import 'package:path/path.dart' as path;
import './bindings.dart' show Calculations;

Calculations loadCalculationsLibrary() {
  final lib = DynamicLibrary.open(_getLibAbsolutePath());
  return Calculations(lib);
}

String _getLibAbsolutePath() =>
    path.join(Directory.current.absolute.path, _getLibRelativePath());

// Relative to dart_client root when `dart run dart_client.dart` is run there.
String _getLibRelativePath() {
  if (Platform.isMacOS) {
    return '../calculations/target/release/libcalculations.dylib';
  } else if (Platform.isWindows) {
    print('Windows OS needs implementation for path to library');
    return 'calculations.dll'; // NOT TESTED
  } else {
    print('Other OS needs implementation for path to library');
    return 'libcalculations.so'; // NOT TESTED
  }
}