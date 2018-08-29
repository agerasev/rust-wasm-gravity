#[macro_use]
extern crate lazy_static;
extern crate physsol;

pub mod console;
#[macro_use]
pub mod macros;
pub mod math;

pub mod app;
pub mod canvas;
pub mod body;

use std::sync::Mutex;

use app::App;

extern {
    #[allow(dead_code)]
    fn js_timeout(sec: f64);
}

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
pub extern fn timeout(dt: f64) {
    my_print!("timeout: {} sec", dt);
}

#[no_mangle]
pub extern fn render(dt: f64) {
    let mut guard = APP.lock().unwrap();
    let app = guard.as_mut().unwrap();

    app.step(dt);
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
