use crate::codegen::compile_error;
use proc_macro::{Span, TokenStream, TokenTree};

fn wrong_input() -> Result<TokenStream, TokenStream> {
    compile_error(
        Span::call_site(),
        "Wrong input passed to macro; did you try to use the termcolor_output_impl directly?",
    )
}
/// Parse input, assuming that it has the known form.
fn parse_wrapper(input: TokenStream) -> Result<TokenStream, TokenStream> {
    match input.into_iter().nth(2) {
        Some(TokenTree::Group(group)) => match group.stream().into_iter().nth(2) {
            Some(TokenTree::Group(group)) => match group.stream().into_iter().nth(2) {
                Some(TokenTree::Group(group)) => Ok(group.stream()),
                _ => wrong_input(),
            },
            _ => wrong_input(),
        },
        _ => wrong_input(),
    }
}

pub fn parse_input(input: TokenStream) -> Result<(String, Vec<TokenStream>), TokenStream> {
    let items = parse_tokens(parse_wrapper(input)?);
    let mut items_iter = items.iter().cloned();
    let writer_expr = match items_iter.next() {
        Some(f) => f,
        None => compile_error(
            Span::call_site(),
            "colored! macro can't be called without arguments",
        )?,
    };

    let format_token = match items_iter.next() {
        Some(f) => f,
        None => compile_error(
            Span::call_site(),
            if writer_expr.to_string().starts_with('"') {
                "The first argument to colored! macro can't be a string. Did you forget to provide the Writer?"
            } else {
                "colored! macro requires at least two arguments - writer and format string"
            },
        )?,
    };
    let format = format_token.to_string();
    eprintln!("format = {:?}", format);

    if !format.starts_with('"') {
        compile_error(
            format_token.into_iter().next().unwrap().span(),
            "The second argument must be a literal string",
        )?;
    }

    Ok((format, items))
}

fn parse_tokens(input: TokenStream) -> Vec<TokenStream> {
    let input = input.into_iter();
    let mut args = vec![];
    let mut cur = vec![];
    for tok in input {
        if let TokenTree::Punct(punct) = tok.clone() {
            if punct.as_char() == ',' {
                args.push(cur.drain(..).collect());
                continue;
            }
        }
        cur.push(tok);
    }
    if !cur.is_empty() {
        args.push(cur.into_iter().collect());
    }
    args
}
