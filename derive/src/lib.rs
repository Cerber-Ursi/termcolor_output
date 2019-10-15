extern crate proc_macro;

use proc_macro::TokenStream;

mod codegen;
mod types;
mod parse;

use types::*;

use codegen::*;
use parse::parse_input;

#[proc_macro_derive(ColoredOutput)]
pub fn colored_derive(input: TokenStream) -> TokenStream {
    let input = parse_input(input);
    match input {
        Ok(_) => unimplemented!(),
        Err(body) => macro_wrapper(compile_error(body)),
    }
}
