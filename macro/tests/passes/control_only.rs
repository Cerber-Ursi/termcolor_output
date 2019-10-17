use termcolor_output::colored;

fn main() {
    let w: Vec<u8> = vec![];
    colored!(w, "{} {} {} {} {} {} {}", bold!(true), underline!(true), bold!(false), fg!(Color::White), bg!(Color::Black), reset!(), intense!(true));
}
