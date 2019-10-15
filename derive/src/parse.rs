use proc_macro::{Span, TokenStream, TokenTree};
use crate::{CompileError, MacroInput};

fn wrong_input() -> CompileError {
    (
        Span::call_site(),
        "Wrong input passed to macro; did you try to use the termcolor_output_impl directly?",
    )
}
/// Parse input, assuming that it has the known form.
fn parse_wrapper(input: TokenStream) -> Result<TokenStream, CompileError> {
    match input.into_iter().nth(2) {
        Some(TokenTree::Group(group)) => match group.stream().into_iter().nth(2) {
            Some(TokenTree::Group(group)) => match group.stream().into_iter().nth(2) {
                Some(TokenTree::Group(group)) => Ok(group.stream()),
                _ => Err(wrong_input()),
            },
            _ => Err(wrong_input()),
        },
        _ => Err(wrong_input()),
    }
}

pub fn parse_input(input: TokenStream) -> Result<MacroInput, CompileError> {
    let mut items = parse_tokens(parse_wrapper(input)?).into_iter();
    let writer = match items.next() {
        Some(f) => f,
        None => return Err((
            Span::call_site(),
            "colored! macro can't be called without arguments",
        )),
    };

    let format_token = match items.next() {
        Some(f) => f,
        None => return Err((
            Span::call_site(),
            if writer.to_string().starts_with('"') {
                "The first argument to colored! macro can't be a string. Did you forget to provide the Writer?"
            } else {
                "colored! macro requires at least two arguments - writer and format string"
            },
        )),
    };
    let format = format_token.to_string();
    eprintln!("format = {:?}", format);

    if !format.starts_with('"') {
        return Err((
            format_token.into_iter().next().unwrap().span(),
            "The second argument must be a literal string",
        ));
    }

    Ok(MacroInput { writer, format, rest: items.collect() })
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
