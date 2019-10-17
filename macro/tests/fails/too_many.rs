use termcolor::NoColor;
use termcolor_output::colored;

fn main() {
    let mut w: NoColor<Vec<u8>> = NoColor::new(vec![]);
    match colored!(w, "", Some(()), vec![1u8]) {
        Ok(_) => {}
        Err(_) => {}
    };
}
