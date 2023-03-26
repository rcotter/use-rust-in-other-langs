public class CalculationsJNI {
    static {
        try {
            System.loadLibrary("calculations"); 
        } catch (UnsatisfiedLinkError e) {
            System.err.println("Native code library failed to load.\n" + e);
            System.exit(1);
        }
    }

    public native double add(double a, double b);
}
