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
    // let earth_radius_kilometer = 6371.0_f64;
    // let central_angle_inner = ((from_lat - to_lat).to_radians() / 2.0).sin().powi(2)
    //     + from_lat.to_radians().cos() * to_lat.to_radians().cos() * ((from_long - to_long).to_radians() / 2.0).sin().powi(2);
    // let central_angle = 2.0 * central_angle_inner.sqrt().asin();
    // return earth_radius_kilometer * central_angle;
    println!("From inside calculation library");
}

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}