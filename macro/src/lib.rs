trait ColoredOutput {}

#[macro_export]
macro_rules! colored {
    ($($arg:tt)*) => {{
        use termcolor_output_impl::ColoredOutput;
        #[derive(ColoredOutput)]
        enum __Writer {
            data = (stringify!($($arg)*), 0).1
        }
        colored_impl!();
    }}
}
