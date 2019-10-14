extern crate proc_macro;

use proc_macro::TokenStream;

mod codegen;
mod parse;

use codegen::macro_wrapper;
use parse::parse_input;

#[proc_macro_derive(ColoredOutput)]
pub fn colored_derive(input: TokenStream) -> TokenStream {
    let input = parse_input(input);
    if let Err(body) = input {
        return macro_wrapper(body);
    }

    unimplemented!();
}
