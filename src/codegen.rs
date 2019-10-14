use crate::formatter::FormatterKind;
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing::*, Span, TokenStream, TokenTree};

#[derive(Debug)]
pub struct Arg {
    pub kind: Option<FormatterKind>,
    pub expr: TokenStream,
}

pub fn func(args: Vec<Arg>, body: TokenStream) -> TokenStream {
    eprintln!("{:?}", args);
    vec![
        TokenTree::Ident(Ident::new("fn", Span::call_site())),
        TokenTree::Ident(Ident::new("write", Span::call_site())),
        TokenTree::Punct(Punct::new('<', Alone)),
    ]
    .into_iter()
    .chain(args_to_types(&args).into_iter())
    .chain(
        vec![
            TokenTree::Punct(Punct::new('>', Alone)),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, args_to_stream(&args))),
            TokenTree::Group(Group::new(Delimiter::Brace, body)),
        ]
        .into_iter(),
    )
    .collect()
}

fn args_to_types(args: &[Arg]) -> TokenStream {
    args.into_iter()
        .enumerate()
        .flat_map(|(index, arg)| {
            // TODO - what Span should we use here?
            let mut ty = vec![TokenTree::Ident(Ident::new(
                &format!("T{}", index),
                Span::call_site(),
            ))];
            if let Some(kind) = arg.kind.clone() {
                if let FormatterKind::Unknown(_) = kind {
                } else {
                    ty.push(TokenTree::Punct(Punct::new(':', Alone)));
                    ty.push(TokenTree::Ident(Ident::new(
                        match kind {
                            FormatterKind::Debug => "Debug",
                            FormatterKind::Display => "Display",
                            FormatterKind::Unknown(_) => unreachable!(),
                        },
                        Span::call_site(),
                    )));
                }
            }
            ty
        })
        .collect()
}

fn args_to_stream(args: &[Arg]) -> TokenStream {
    args.into_iter()
        .enumerate()
        .flat_map(|(index, _)| {
            vec![
                TokenTree::Ident(Ident::new(&format!("_arg{}", index), Span::call_site())),
                TokenTree::Punct(Punct::new(':', Alone)),
                TokenTree::Ident(Ident::new(&format!("T{}", index), Span::call_site())),
            ]
        })
        .collect()
}
