pub mod ffi;

extern "C" {
    pub fn moving_average(data: *const f32, length: i32) -> f32;
}