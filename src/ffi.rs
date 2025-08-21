// filepath: /Users/admin/Documents/stock-analyzer/src/ffi.rs
extern "C" {
    pub fn moving_average(data: *const f32, length: i32) -> f32;
}