from ctypes import cdll

# Dynamically linked
lib = cdll.LoadLibrary("/Users/rick/Documents/src/wolfepak-calculations/calculations/target/release/libcalculations.dylib")
lib.distance_between_two_points_on_the_earth()
print(f"2 + 3 = {lib.add(2, 3)}")
print("done!")