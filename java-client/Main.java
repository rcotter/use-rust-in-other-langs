class Main {    
    public static void main(String[] args) {
        System.out.println("Running Java main()");
        
        CalculationsJNI calculations = new CalculationsJNI();
        System.out.println("5.0 + 7.0 = " + calculations.add(5, 7));
        System.out.println("2.2 + 3.3 = " + calculations.add(2.2, 3.3));
    }
}
