use crate::{CompileError, FormatItems, FormatPart, MacroInput};
use proc_macro::{Span, TokenStream, TokenTree};

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
        None => {
            return Err((
                Span::call_site(),
                "colored! macro can't be called without arguments",
            ))
        }
    };

    let format_token = match items.next() {
        Some(f) => f,
        None => {
            return Err((
                Span::call_site(),
                if writer.to_string().starts_with('"') {
                    "The first argument to colored! macro can't be a string. Did you forget to provide the Writer?"
                } else {
                    "colored! macro requires at least two arguments - writer and format string"
                },
            ))
        }
    };
    let format = parse_format_string(format_token)?;
    eprintln!("{:?}", format);

    Ok(MacroInput {
        writer,
        format,
        rest: items.collect(),
    })
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

fn parse_format_string(input: TokenStream) -> Result<FormatItems, CompileError> {
    let mut input = input.into_iter();
    let format_token = match input.next() {
        Some(tok) => tok,
        None => {
            return Err((
                Span::call_site(),
                "Expected format string, got empty stream",
            ))
        }
    };
    match input.next() {
        None => {}
        Some(tok) => {
            return Err((
                tok.span(),
                "Unexpected token, did you forget the comma after format string?",
            ))
        }
    };
    let span = format_token.span();
    match format_token {
        TokenTree::Literal(_) => {}
        _ => return Err((span, "The second argument must be a literal string")),
    };
    let format = format_token.to_string();
    if !format.starts_with('"') {
        return Err((span, "The second argument must be a literal string"));
    }
    let format = format.trim_matches('"');

    let mut parts = vec![];
    // this will usually overallocate, but the format string isn't going to be large anyway
    let mut cur = String::with_capacity(format.len());
    let mut in_format = false;
    let mut chars = format.chars();

    while let Some(ch) = chars.next() {
        if in_format && ch == '}' {
            in_format = false;
            parts.push(FormatPart::Input(cur.clone() + "}"));
            cur.clear();
        } else if !in_format && ch == '{' {
            if let Some(next) = chars.next() {
                if next == '{' {
                    cur.push(next);
                } else {
                    parts.push(FormatPart::Text(cur.clone()));
                    if next == '}' {
                        // special case, handled right here
                        parts.push(FormatPart::Input("{}".into()));
                        cur.clear();
                    } else {
                        in_format = true;
                    
                    cur.clear();
                    cur.push('{');
                cur.push(next);
                    }
                }
            } else {
                return Err((span, "Unexpected end of format string"));
            }
        } else if ch == '}' {
            // in_format guaranteed to be false
            if let Some(next) = chars.next() {
                if next == '}' {
                    cur.push(next);
                } else {
                    return Err((span, "Unmatched '}' in format string; did you forget to escape it as '}}'?"));
                }
            } else {
                return Err((span, "Unexpected end of format string"));
            }
        } else {
            cur.push(ch);
        }
    }
    if !cur.is_empty() {
        parts.push(if in_format { FormatPart::Input } else { FormatPart::Text }(cur));
    }

    Ok(FormatItems { span, parts })
}
