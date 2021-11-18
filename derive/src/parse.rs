use crate::*;
use proc_macro::{Span, TokenStream, TokenTree};

pub fn parse_input(input: TokenStream) -> Result<MacroInput> {
    let mut items = parse_tokens(input)?.into_iter();
    let writer = match items.next() {
        Some(f) => f,
        None => {
            return Err((
                Span::call_site(),
                Span::call_site(),
                "colored! macro can't be called without arguments",
            ))
        }
    };

    let format_token = match items.next() {
        Some(f) => f,
        None => {
            let vec: Vec<_> = writer.clone().into_iter().collect();
            return Err((
                vec[0].span(),
                vec[vec.len() - 1].span(),
                if writer.to_string().starts_with('"') {
                    "The first argument to colored! macro can't be a string. Did you forget to provide the Writer?"
                } else {
                    "colored! macro requires at least two arguments - writer and format string"
                },
            ));
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
                    return Err((
                        punct.span(),
                        punct.span(),
                        "Unexpected ',', expected expression",
                    ));
                } else {
                    args.push(cur.drain(..).collect());
                }
            }
        } else {
            cur.push(tok);
        }
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
            Span::call_site(),
            concat!("Empty token stream in 'classify'; this is supposed to be unreachable. Please report this case to ", env!("CARGO_PKG_REPOSITORY"), "/issues") 
        )
    )?;
    match iter.next() {
        Some(TokenTree::Punct(ref punct)) if punct.as_char() == '!' => {
            match iter.next() {
                Some(TokenTree::Group(ref group)) => {
                    let inner = match first.to_string().as_str() {
                        "reset" => ControlSeq::Reset,
                        x if is_format_control(x) => ControlSeq::Command(x.into(), group.stream()),
                        // well, maybe it is some external macro like `vec!` or `env!`
                        _ => return Ok(InputItem::Raw(input)),
                    };
                    Ok(InputItem::Ctrl(inner))
                }
                // if it's something like the macro, but not quite, - let it error out after
                // expanding
                _ => Ok(InputItem::Raw(input)),
            }
        }
        _ => Ok(InputItem::Raw(input)),
    }
}

fn is_format_control(s: &str) -> bool {
    s == "bold" || s == "underline" || s == "intense" || s == "fg" || s == "bg"
}

fn parse_format_string(input: TokenStream) -> Result<FormatItems> {
    let mut input = input.into_iter();
    let format_token = match input.next() {
        Some(tok) => tok,
        None => {
            return Err((
                // TODO: maybe get spans of two adjacent commas?
                Span::call_site(),
                Span::call_site(),
                "Expected format string, got empty stream",
            ));
        }
    };
    match input.next() {
        None => {}
        Some(tok) => {
            return Err((
                tok.span(),
                tok.span(),
                "Unexpected token, did you forget the comma after format string?",
            ))
        }
    };
    let span = format_token.span();
    match format_token {
        TokenTree::Literal(_) => {}
        _ => return Err((span, span, "The second argument must be a literal string")),
    };
    let format = format_token.to_string();
    if !format.starts_with('"') {
        return Err((span, span, "The second argument must be a literal string"));
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
                    if !cur.is_empty() {
                        parts.push(FormatPart::Text(cur.clone()));
                    }
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
                return Err((span, span, "Unexpected end of format string"));
            }
        } else if ch == '}' {
            // in_format guaranteed to be false
            if let Some(next) = chars.next() {
                if next == '}' {
                    cur.push(next);
                } else {
                    return Err((
                        span,
                        span,
                        "Unmatched '}' in format string; did you forget to escape it as '}}'?",
                    ));
                }
            } else {
                return Err((span, span, "Unexpected end of format string"));
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
                // TODO - carry spans with Ctrl?
                pull_format(
                    &mut format,
                    &mut cur_format,
                    Span::call_site(),
                    Span::call_site(),
                )?;
                flush(&mut cur_format, &mut cur_items, &mut output);
                output.push(OutputItem::Ctrl(ctrl));
            }
            // If this is normal item, we pull items from format string until we get the corresponding
            // format specifier.
            Some(InputItem::Raw(raw)) => {
                let mut iter = raw.clone().into_iter();
                let start = iter
                    .next()
                    .map_or(Span::call_site(), |t| TokenTree::span(&t));
                let end = iter.last().map_or(start, |t| TokenTree::span(&t));
                let part = pull_format(&mut format, &mut cur_format, start, end)?;
                cur_format.push_str(&part);
                cur_items.push(raw);
            }
            // If the items vector is exhausted, well, we're either OK, or have too long format string.
            None => match pull_format(
                &mut format,
                &mut cur_format,
                Span::call_site(),
                Span::call_site(),
            ) {
                // We have to effectively "reverse" the result, since if we've successfully pulled
                // something, it means that we've got something unexpected.
                Ok(_) => {
                    return Err((
                        format_span,
                        format_span,
                        "Not enough input parameters for this format string",
                    ))
                }
                Err(_) => {
                    flush(&mut cur_format, &mut cur_items, &mut output);
                    break;
                }
            },
        }
    }
    Ok(output)
}

fn flush(format: &mut String, items: &mut Vec<TokenStream>, output: &mut Vec<OutputItem>) {
    if !format.is_empty() {
        output.push(OutputItem::Raw((
            format.drain(..).collect(),
            items.drain(..).collect(),
        )));
    }
}

fn pull_format(
    format: &mut impl Iterator<Item = FormatPart>,
    cur_format: &mut String,
    start: Span,
    end: Span,
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
                return Err((
                    start,
                    end,
                    "Too many input parameters for this format string",
                ));
            }
        }
    }
}
