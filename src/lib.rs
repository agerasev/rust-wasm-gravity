#[macro_use]
extern crate lazy_static;
extern crate rand;
#[macro_use]
extern crate wasm_env as wasm;
extern crate physsol;

pub mod app;
pub mod body;

use app::App;

wasm_bind!(wasm, || Box::new(App::new()));
