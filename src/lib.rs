extern crate rand;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate wasm_env as wasm;
extern crate physsol;

pub mod app;
pub mod body;

use app::App;

bind_wasm!(App, wasm);
