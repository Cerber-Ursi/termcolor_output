extern crate proc_macro;

use proc_macro::TokenStream;

mod codegen;
mod parse;
mod types;

use codegen::*;
use parse::*;
use types::*;

#[proc_macro_derive(ColoredOutput)]
pub fn colored_derive(input: TokenStream) -> TokenStream {
    let body = parse_input(input)
        .and_then(|input| {
            let MacroInput {
                writer,
                format,
                rest,
            } = input;

            let output_items = merge_items(format, rest)?
                .into_iter()
                .map(output)
                .collect::<Result<Vec<_>>>()?;

            Ok(guard(writer)
                .into_iter()
                .chain(spec_init().into_iter())
                .chain(output_items.into_iter().flat_map(TokenStream::into_iter))
                .collect())
        })
        .unwrap_or_else(|err| compile_error(err));
    let out = macro_wrapper(closure_wrapper(body));
    // eprintln!("{}", proc_macro2::TokenStream::from(out.clone()));
    out
}
