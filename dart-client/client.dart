import './lib/load_calculations_library.dart';

void main() {
  final calc = loadCalculationsLibrary();

  double left = 2.2;
  double right = 3.3;
  double sum = calc.add(left, right); // From Rust sourced C lib
  print('$left + $right = $sum');
}