use jni::sys::*;
use jni::objects::JClass;
use jni::JNIEnv;

// 'jni' looks for CalculationsJNI existing in the 'include' folder.

#[no_mangle]
pub extern fn add(a: f64, b: f64) -> f64 {
    a + b
}

// Implement the JNI function for use from Java
#[no_mangle]
pub extern "system" fn Java_CalculationsJNI_add(
    _env: JNIEnv,
    _class: JClass,
    a: jdouble,
    b: jdouble,
) -> jdouble { add(a, b) as jdouble }

#[no_mangle]
pub extern fn distance_between_two_points_on_the_earth() {
    println!("From inside calculation library");
}