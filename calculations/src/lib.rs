// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

// Calculate the distance between two points in the Earth.
// For example, from Paris (48.85341_f64, -2.34880_f64) to London (51.50853_f64, -0.12574_f64)
// // https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/trigonometry.html#distance-between-two-points-on-the-earth

// pub extern fn distance_between_two_points_on_the_earth(from_lat: f64, from_long: f64, to_lat: f64, to_long: f64) -> f64 {
#[no_mangle]
pub extern fn distance_between_two_points_on_the_earth() {
    println!("From inside calculation library");
}

#[no_mangle]
pub extern fn add(a: i32, b: i32) -> i32 {
    a + b
}