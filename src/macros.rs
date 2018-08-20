macro_rules! my_print(
    ($($arg:tt)*) => { {
        console::write(console::Kind::Log, &format!($($arg)*));
    } }
);

macro_rules! my_eprint(
    ($($arg:tt)*) => { {
        console::write(console::Kind::Error, &format!($($arg)*));
    } }
);
