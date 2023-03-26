from ctypes import cdll, c_double

# Dynamically linked
lib = cdll.LoadLibrary("/Users/rick/Documents/src/use-rust-in-other-langs/calculations/target/release/libcalculations.dylib")
lib.distance_between_two_points_on_the_earth()

# Tell python how to map C types
lib.add.argtypes = (c_double, c_double)
lib.add.restype = c_double

print(f"2 + 3 = {lib.add(2, 3)}")
print(f"2.2 + 3.3 = {lib.add(2.2, 3.3)}")
print("done!")