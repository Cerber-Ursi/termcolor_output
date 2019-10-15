use termcolor_output::*;

fn main() {
    let writer: Vec<u8> = vec![];
    colored!(writer, "Before first - {}, then - {:?}, or even {{this}}: {:b}!");
}
