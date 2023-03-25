cp ../calculations/target/release/libcalculations.dylib lib/libcalculations.dylib & cp ../calculations/target/calculations.hpp lib/calculations.h & javac Main.java & java -Djava.library.path=lib Main
