use jni::sys::*;
use jni::objects::JClass;
use jni::JNIEnv;

// // Include the JNI header file
// #[allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
// mod calculationsjni {
//     include!("../include/CalculationsJNI.h");
// }

// Implement the JNI function
#[no_mangle]
pub extern "system" fn Java_CalculationsJNI_add(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    a + b // Duplicated below
}

#[no_mangle]
pub extern fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern fn distance_between_two_points_on_the_earth() {
    println!("From inside calculation library");
}