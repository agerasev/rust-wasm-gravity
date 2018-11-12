use wasm::types::Type;
use wasm::module::Module;
use wasm::canvas::Canvas;

#[derive(Debug)]
pub enum UserEvent {
    Pause(bool),
    Resize,
}

impl UserEvent {
    pub fn from(mut data: &[u8]) -> Result<UserEvent, ()> {
        let r = &mut data;
        match u32::load(r).unwrap() {
            0x01 => Ok(UserEvent::Pause(i32::load(r).unwrap() != 0)),
            0x02 => Ok(UserEvent::Resize),
            _ => Err(()),
        }
    }
}

pub struct Helper {
    pub module: Module,
}

impl Helper {
    pub fn new(module: Module) -> Self {
        Self { module }
    }
    pub fn set_screen(&mut self, canvas: &Canvas) {
        let mut buf = vec!(0 as u8; 4);
        canvas.id().store(&mut (&mut buf as &mut [u8])).unwrap();
        self.module.call("set_screen", &mut buf).unwrap();
    }
}
