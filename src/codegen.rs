use crate::formatter::FormatterKind;
use proc_macro::{
    Delimiter::*, Group, Ident, Literal, Punct, Spacing::*, Span, TokenStream, TokenTree,
};

#[derive(Debug)]
pub struct Arg {
    pub kind: Option<FormatterKind>,
    pub expr: TokenStream,
}

macro_rules! tt {
    (Literal::$ty:tt($($args:expr),*)) => {
        TokenTree::Literal(Literal::$ty($($args),*))
    };
    ($ty:tt($($args:expr),*)) => {
        TokenTree::$ty($ty::new($($args),*))
    };
}

pub fn macro_wrapper(body: TokenStream) -> TokenStream {
    vec![
        tt!(Ident("macro_rules", Span::call_site())),
        tt!(Punct('!', Alone)),
        tt!(Ident("colored_impl", Span::call_site())),
        tt!(Group(
            Brace,
            vec![
                tt!(Group(Parenthesis, TokenStream::new())),
                tt!(Punct('=', Joint)),
                tt!(Punct('>', Alone)),
                tt!(Group(Brace, body))
            ]
            .into_iter()
            .collect()
        )),
    ]
    .into_iter()
    .collect()
}

pub fn compile_error(start: Span, error: &str) -> TokenStream {
    vec![
        tt!(Ident("compile_error", start)),
        tt!(Punct('!', Alone)),
        tt!(Group(
            Parenthesis,
            vec![
                // TODO span
                tt!(Literal::string(error)),
            ]
            .into_iter()
            .collect()
        )),
    ]
    .into_iter()
    .collect()
}
