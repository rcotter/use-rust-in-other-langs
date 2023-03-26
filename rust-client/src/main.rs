use calculations::add;
use calculations::distance_between_two_points_on_the_earth;

fn main() {
    distance_between_two_points_on_the_earth();
    println!("2 + 3 = {}", add(2.0, 3.0));
    println!("2.2 + 3.3 = {}", add(2.2, 3.3));
}
