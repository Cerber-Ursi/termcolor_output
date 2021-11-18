//! Procedural macro crate for [termcolor_output].
//!
//! Please do not depend on this crate directly.
//!
//! [termcolor_output]: http://docs.rs/termcolor_output

extern crate proc_macro;

use proc_macro::TokenStream;

mod codegen;
mod parse;
mod types;

use codegen::*;
use parse::*;
use types::*;

#[proc_macro]
pub fn colored_generate(input: TokenStream) -> TokenStream {
    let body = parse_input(input)
        .and_then(|input| {
            let MacroInput {
                writer,
                format,
                rest,
            } = input;

            Ok(guard(writer)
                .into_iter()
                .chain(spec_init().into_iter())
                .chain(
                    merge_items(format, rest)?
                        .into_iter()
                        .map(output)
                        .flat_map(TokenStream::into_iter),
                )
                .collect())
        })
        .unwrap_or_else(compile_error);
    macro_wrapper(closure_wrapper(body))
}
