javac -h . CalculationsJNI.java &
cp CalculationsJNI.h ../calculations/include

echo "JNI header CalculationsJSN.h generated and copied to the Rust library."
echo "The Rust library requires a rebuild before running this Java app."