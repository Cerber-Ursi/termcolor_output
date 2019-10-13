pub use termcolor_output_impl::colored as colored_impl;

#[macro_export]
macro_rules! colored {
    ($($arg:tt),*) => {{
        struct __Writer;
        impl __Writer {
            colored_impl!($($arg),*);
        }
        __Writer::write($(&($arg)),*);
    }}
}
