use termcolor::NoColor;
use termcolor_output::colored;

fn main() {
    let mut w: NoColor<Vec<u8>> = NoColor::new(vec![]);
    match colored!(
        w,
        "Text: {}\nText (escaped): {:?}\nNumbers pair: {:?}\nOption: {:#?}",
        "Text 1",
        "Text 2 - with \"something else\", \\-_-/!",
        (100, 100.1),
        Some(Some(Some(())))
    ) {
        Ok(_) => {}
        Err(_) => {}
    };
}
