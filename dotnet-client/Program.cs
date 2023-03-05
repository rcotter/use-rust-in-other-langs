using System.Runtime.InteropServices;

[DllImport("../calculations/target/release/libcalculations.dylib", CallingConvention = CallingConvention.Cdecl)]
static extern int add(int a, int b);

[DllImport("../calculations/target/release/libcalculations.dylib", CallingConvention = CallingConvention.Cdecl)]
static extern void distance_between_two_points_on_the_earth();

distance_between_two_points_on_the_earth();
Console.WriteLine($"2 + 3 = {add(2, 3)}");
