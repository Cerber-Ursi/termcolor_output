extern crate termcolor_output_impl;
pub trait ColoredOutput {}

#[macro_export]
macro_rules! colored {
    ($($arg:tt)*) => {{
        use $crate::ColoredOutput;
        #[derive(ColoredOutput)]
        enum __Writer {
            data = (stringify!($($arg)*), 0).1
        }
        colored_impl!();
    }}
}
