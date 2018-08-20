mod console;

#[macro_use]
mod macros;

#[no_mangle]
pub extern fn main() {
    console::setup();
    let a = [1, 2, 3];
    let i = 3;
    my_print!("{:?}", a[i]);
}
