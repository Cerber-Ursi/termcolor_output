use crate::*;
use proc_macro::{Span, TokenStream, TokenTree};

fn wrong_input() -> CompileError {
    (
        Span::call_site(),
        "Wrong input passed to macro; did you try to use the termcolor_output_impl directly?",
    )
}
/// Parse input, assuming that it has the known form.
fn parse_wrapper(input: TokenStream) -> Result<TokenStream> {
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

pub fn parse_input(input: TokenStream) -> Result<MacroInput> {
    let mut items = parse_tokens(parse_wrapper(input)?)?.into_iter();
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

    Ok(MacroInput {
        writer,
        format,
        rest: items.map(classify_format_arg).collect::<Result<_>>()?,
    })
}

fn parse_tokens(input: TokenStream) -> Result<Vec<TokenStream>> {
    let input = input.into_iter();
    let mut args = vec![];
    let mut cur = vec![];
    for tok in input {
        if let TokenTree::Punct(punct) = tok.clone() {
            if punct.as_char() == ',' {
                if cur.is_empty() {
                    return Err((punct.span(), "Unexpected ',', expected expression"));
                } else {
                    args.push(cur.drain(..).collect());
                    continue;
                }
            }
        }
        cur.push(tok);
    }
    if !cur.is_empty() {
        args.push(cur.into_iter().collect());
    }
    Ok(args)
}

fn classify_format_arg(input: TokenStream) -> Result<InputItem> {
    let mut iter = input.clone().into_iter();
    let first = iter.next().ok_or(
        (
            Span::call_site(),
            concat!("Empty token stream in 'classify'; this is supposed to be unreachable. Please report this case to ", env!("CARGO_PKG_REPOSITORY"), "/issues") 
        )
    )?;
    match iter.next() {
        Some(TokenTree::Punct(ref punct)) if punct.as_char() == '!' => {
            match iter.next() {
                Some(TokenTree::Group(ref group)) => {
                    let inner = match first.to_string().as_str() {
                        "bold" => ControlSeq::Bold,
                        "underline" => ControlSeq::Underline,
                        "intense" => ControlSeq::Intense,
                        "fg" => ControlSeq::Foreground,
                        "bg" => ControlSeq::Background,
                        "reset" => return Ok(InputItem::Ctrl(ControlSeq::Reset)),
                        // well, maybe it is some external macro like `vec!` or `env!`
                        _ => return Ok(InputItem::Raw(input)),
                    };
                    Ok(InputItem::Ctrl(inner(group.stream())))
                }
                // if it's something like the macro, but not quite, - let it error out after
                // expanding
                _ => Ok(InputItem::Raw(input)),
            }
        }
        _ => Ok(InputItem::Raw(input)),
    }
}

fn parse_format_string(input: TokenStream) -> Result<FormatItems> {
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
    // this will usually over-allocate, but the format string isn't going to be large anyway
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
                    return Err((
                        span,
                        "Unmatched '}' in format string; did you forget to escape it as '}}'?",
                    ));
                }
            } else {
                return Err((span, "Unexpected end of format string"));
            }
        } else {
            cur.push(ch);
        }
    }
    if !cur.is_empty() {
        parts.push(if in_format {
            FormatPart::Input
        } else {
            FormatPart::Text
        }(cur));
    }

    Ok(FormatItems { span, parts })
}

pub fn merge_items(format: FormatItems, input: Vec<InputItem>) -> Result<Vec<OutputItem>> {
    let format_span = format.span;
    let mut format = format.parts.into_iter();
    let mut input = input.into_iter();
    let mut output = vec![];
    let mut cur_format = String::new();
    let mut cur_items = vec![];
    loop {
        // We pull items from the input one at a time.
        match input.next() {
            // If we've got a control sequence, we must flush all aggregated raw output
            // and then push the sequence itself.
            Some(InputItem::Ctrl(ctrl)) => {
                pull_format(&mut format, &mut cur_format, Span::call_site())?;
                if !cur_format.is_empty() {
                    output.push(OutputItem::Raw((
                        cur_format.drain(..).collect(),
                        cur_items.drain(..).collect(),
                    )));
                    output.push(OutputItem::Ctrl(ctrl));
                }
            }
            // If this is normal item, we pull items from format string until we get the corresponding
            // format specifier.
            Some(InputItem::Raw(raw)) => {
                let part = pull_format(
                    &mut format,
                    &mut cur_format,
                    raw.into_iter().next().unwrap().span(),
                )?;
                cur_format.push_str(&part);
            }
            // If the items vector is exhausted, well, we're either OK, or have too long format string.
            None => match format.next() {
                None => {
                    if !cur_format.is_empty() {
                        output.push(OutputItem::Raw((
                            cur_format.drain(..).collect(),
                            cur_items.drain(..).collect(),
                        )));
                    }
                    break;
                }
                Some(_) => {
                    return Err((
                        format_span,
                        "Not enough input parameters for this format string",
                    ))
                }
            },
        }
    }
    Ok(output)
}

fn pull_format(
    format: &mut impl Iterator<Item = FormatPart>,
    cur_format: &mut String,
    span: Span,
) -> Result<String> {
    loop {
        match format.next() {
            Some(FormatPart::Input(s)) => {
                return Ok(s);
            }
            Some(FormatPart::Text(ref s)) => {
                cur_format.push_str(s);
            }
            None => {
                return Err((span, "Too many input parameters for this format string"));
            }
        }
    }
}
