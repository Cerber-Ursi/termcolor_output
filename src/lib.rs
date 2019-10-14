extern crate proc_macro;

use proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing::*, Span, TokenStream, TokenTree,
};

mod codegen;
mod formatter;

use codegen::Arg;

#[proc_macro]
pub fn colored(input: TokenStream) -> TokenStream {
    let input = parse_input(input);
    if let Err((args, body)) = input {
        return codegen::func(args, body);
    }

    unimplemented!();
}

fn parse_input(input: TokenStream) -> Result<(String, Vec<Arg>), (Vec<Arg>, TokenStream)> {
    let mut tokens = input.into_iter();
    let writer_expr = match tokens.next() {
        Some(f) => f,
        None => Err((
            vec![],
            compile_error(
                Span::call_site(),
                "colored! macro can't be called without arguments",
            ),
        ))?,
    };

    let format_token = match tokens.next() {
        Some(f) => f,
        None => Err((
            vec![],
            compile_error(
                Span::call_site(),
                if writer_expr.to_string().starts_with("\"") {
                    "The first argument to colored! macro can't be a string. Did you forget to provide the Writer?"
                } else {
                    "colored! macro requires at least two arguments - writer and format string"
                }
            ),
        ))?,
    };
    let format = format_token.to_string();
    eprintln!("format = {:?}", format);
    let args = parse_tokens(format_token.clone(), tokens);
    if !format.starts_with('"') {
        return Err((
            args,
            compile_error(
                format_token.span(),
                "The second argument must be a literal string",
            ),
        ));
    }

    Ok((format, args))
}

fn parse_tokens(writer: TokenTree, mut input: proc_macro::token_stream::IntoIter) -> Vec<Arg> {
    let mut args = vec![Arg {kind: None, expr: TokenStream::new()}];
    let mut cur = vec![];
    while let Some(tok) = input.next() {
        if let TokenTree::Punct(punct) = tok.clone() {
            if punct.as_char() == ',' {
                args.push(Arg {
                    kind: None,
                    expr: cur.iter().cloned().collect(),
                });
                continue;
            }
        }
        cur.push(tok);
    }
    args
}

fn compile_error(start: Span, error: &str) -> TokenStream {
    vec![
        TokenTree::Ident(Ident::new("compile_error", start)),
        TokenTree::Punct(Punct::new('!', Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            vec![
                // TODO span
                TokenTree::Literal(Literal::string(error)),
            ]
            .into_iter()
            .collect(),
        )),
    ]
    .into_iter()
    .collect()
}
