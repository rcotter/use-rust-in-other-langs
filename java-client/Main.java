class Main {    
    static {
        System.loadLibrary("calculations"); 
    }

    private static native int add(int a, int b);

    public static void main(String[] args) {
        System.out.println("Hello, world!");
        System.out.println("Result of add(2, 3): " + add(2, 3));
    }
}
