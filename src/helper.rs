use wasm;
use wasm::types::Type;
use wasm::module::Module;
use wasm::canvas::Canvas;

pub struct Helper {
    pub module: Module,
}

#[derive(Debug)]
pub enum UserEvent {
    Pause(bool),
    Resize,
}

impl Helper {
    pub fn new(module: Module) -> Self {
        Self { module }
    }
    pub fn set_screen(&mut self, canvas: &Canvas) {
        wasm::with_buffer_mut(|mut buf: &mut [u8]| {
            canvas.id().store(&mut buf).unwrap();
        });
        self.module.call("set_screen").unwrap();
    }
    pub fn user_event() -> Result<UserEvent, ()> {
        let mut event = Err(());
        wasm::with_buffer(|mut buf: &[u8]| {
            event = match u32::load(&mut buf).unwrap() {
                0x01 => Ok(UserEvent::Pause(i32::load(&mut buf).unwrap() != 0)),
                0x02 => Ok(UserEvent::Resize),
                _ => Err(()),
            }
        });
        event
    }
}