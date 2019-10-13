use termcolor_output::colored as colored_impl;

macro_rules! colored {
    ($($arg:tt),*) => {{
        struct __Writer;
        impl __Writer {
            colored_impl!($($arg),*);
        }
        __Writer::write();
    }}
}

fn main() {
    colored!();
}