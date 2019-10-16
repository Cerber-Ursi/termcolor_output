extern crate proc_macro;

use proc_macro::TokenStream;

mod codegen;
mod parse;
mod types;

use types::*;

use codegen::*;
use parse::parse_input;

#[proc_macro_derive(ColoredOutput)]
pub fn colored_derive(input: TokenStream) -> TokenStream {
    let input = parse_input(input);
    match input {
        Ok(MacroInput {
            writer,
            format,
            rest,
        }) => {
            eprintln!("{:?}", rest);
            let guard = guard(writer);
            unimplemented!();
        }
        Err(body) => macro_wrapper(compile_error(body)),
    }
}
