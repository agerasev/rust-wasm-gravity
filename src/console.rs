use std::panic;

extern {
    fn js_console(t: i32, ptr: *const u8, len: i32);
}

pub enum Kind {
	Log,
	Error
}

pub fn write(t: Kind, msg: &str) {
    unsafe { js_console(match t { Kind::Log => 0, Kind::Error => 1}, msg.as_ptr(), msg.len() as i32); }
}

pub fn setup() {
	panic::set_hook(Box::new(|panic_info| {
        let payload = panic_info.payload();
        let payload = match payload.downcast_ref::<String>() {
            Some(payload) => payload.clone(),
            None => match payload.downcast_ref::<&str>() {
                Some(payload) => String::from(*payload),
                None => String::new(),
            },
        };
        let location = match panic_info.location() {
            Some(location) => format!(" in file '{}' at line {}", location.file(), location.line()),
            None => String::new(),
        };
        write(Kind::Error, &format!("Panic occured{}\n{}", location, payload));
    }));
}
