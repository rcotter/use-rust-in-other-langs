cp ../calculations/target/release/libcalculations.dylib lib/libcalculations.dylib & 
javac CalculationsJNI.java &
javac Main.java

echo "Rust library copied local. Java project compiled."

java -Djava.library.path="/Users/rick/Documents/src/use-rust-in-other-langs/java-client/lib" Main

