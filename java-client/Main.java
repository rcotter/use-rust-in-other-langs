class Main {    
    public static void main(String[] args) {
        System.out.println("Running Java main()");

        CalculationsJNI calculations = new CalculationsJNI();
        int result = calculations.add(5, 7);
        System.out.println("5 + 7 = " + result);
    }
}
