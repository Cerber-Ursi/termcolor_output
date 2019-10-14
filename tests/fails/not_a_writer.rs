use termcolor_output::*;

fn main() {
    let not_a_writer = 0u32;
    colored!(not_a_writer, "test");
}