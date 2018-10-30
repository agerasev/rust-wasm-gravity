#[macro_use]
extern crate lazy_static;
extern crate rand;
#[macro_use]
extern crate wasm_env as wasm;
extern crate physsol;

mod app;
mod body;
mod helper;

use app::App;

wasm_bind!(wasm, || Box::new(App::new()));
