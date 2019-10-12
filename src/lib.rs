extern crate proc_macro;

use proc_macro::{Delimiter, Group, Ident, Punct, Literal, Spacing::*, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn colored(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let format_token = match tokens.next() {
        Some(f) => f,
        None => return compile_error(Span::call_site(), "colored! macro can't be called without arguments"),
    };
    let format = format_token.to_string();
    if !format.starts_with('"') {
        return compile_error(format_token.span(), "The first argument must be a literal string");
    }

    let rest: Vec<_> = tokens.collect();

    unimplemented!();
}

fn compile_error(start: Span, error: &str) -> TokenStream {
    vec![
        TokenTree::Ident(Ident::new("compile_error", start)),
        TokenTree::Punct(Punct::new('!', Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            vec![
                // TODO span
                TokenTree::Literal(Literal::string(error))
            ].into_iter().collect(),
        )),
    ]
    .into_iter()
    .collect()
}
