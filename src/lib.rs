#[macro_use]
extern crate lazy_static;

pub mod console;
#[macro_use]
pub mod macros;

pub mod app;
pub mod canvas;

use std::sync::Mutex;

use app::App;

lazy_static! {
    static ref APP: Mutex<Option<App>> = Mutex::new(None);
}

#[no_mangle]
pub extern fn init() {
    console::setup();
    let mut guard = APP.lock().unwrap();
    match *guard {
        None => { *guard = Some(App::new()); },
        Some(_) => { my_eprint!("App is already initialized!"); },
    }
}

#[no_mangle]
pub extern fn draw() {
    let mut guard = APP.lock().unwrap();
    let app = guard.as_mut().unwrap();
    app.draw();
}

#[no_mangle]
pub extern fn quit() {
    let mut guard = APP.lock().unwrap();
    match *guard {
        None => { my_eprint!("App is already None!"); },
        Some(_) => { *guard = None; },
    }
}
