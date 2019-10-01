#[no_mangle]
pub extern fn hello() -> *const u8 {
    "Hello from Rust!\0".as_ptr()
}
