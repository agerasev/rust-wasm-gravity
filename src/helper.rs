use wasm;
use wasm::interop::{Type};
use wasm::module::{Module};
use wasm::canvas::{Canvas};

pub struct Helper {
    pub module: Module,
}

impl Helper {
    pub fn new(module: Module) -> Self {
        Self { module }
    }
    pub fn set_screen(&mut self, canvas: &Canvas) {
        wasm::with_buffer(|mut buf: &mut [u8]| {
            canvas.id().store(&mut buf).unwrap();
        });
        self.module.call("set_screen").unwrap();
    }
}