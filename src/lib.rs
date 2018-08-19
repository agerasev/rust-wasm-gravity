extern {
	fn func();
}

#[no_mangle]
pub extern fn add_one(x: i32) -> i32 {
	unsafe { func(); }
    x + 1
}
