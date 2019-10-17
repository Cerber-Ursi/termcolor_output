use termcolor::{Color, NoColor};
use termcolor_output::colored;

struct NonDebuggable;

fn main() {
    let mut w: NoColor<Vec<u8>> = NoColor::new(vec![]);
    match colored!(w, "{}{}{:?}", Some(()), vec![1u8], NonDebuggable) {
        Ok(_) => {}
        Err(_) => {}
    };
}
