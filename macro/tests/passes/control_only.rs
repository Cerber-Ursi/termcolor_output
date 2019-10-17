use termcolor_output::colored;
use termcolor::{Color, NoColor};

fn main() {
    let mut w: NoColor<Vec<u8>> = NoColor::new(vec![]);
    colored!(w, "{}{}{}{}{}{}{}", bold!(true), underline!(true), bold!(false), fg!(Some(Color::White)), bg!(Some(Color::Black)), reset!(), intense!(true));
}
