cp ../calculations/target/release/libcalculations.dylib lib/libcalculations.dylib & 
echo "Rust library copied local" &
javac CalculationsJNI.java &
javac Main.java &
echo "Java project compiled." &
java -Djava.library.path="/Users/rick/Documents/src/use-rust-in-other-langs/java-client/lib" Main

