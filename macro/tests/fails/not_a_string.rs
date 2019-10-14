use termcolor_output::*;

fn main() {
    let writer: Vec<u8> = vec![];
    let not_a_string = 0u32;
    colored!(&mut writer, not_a_string);
}