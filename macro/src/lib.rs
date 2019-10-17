trait ColoredOutput {}

#[doc(hidden)]
pub fn guard(w: &mut impl termcolor::WriteColor) -> &mut impl termcolor::WriteColor {
    w
}

#[macro_export]
macro_rules! colored {
    ($($arg:tt)*) => {{
        use termcolor_output_impl::ColoredOutput;
        use termcolor::WriteColor;
        use std::io::Write;
        #[derive(ColoredOutput)]
        enum __Writer {
            data = (stringify!($($arg)*), 0).1
        }
        colored_impl!()
    }}
}
