use proc_macro::{
    Delimiter::*, Group, Ident, Literal, Punct, Spacing::*, Span, TokenStream, TokenTree,
};

use crate::{CompileError, ControlSeq};

macro_rules! tt {
    (Literal::$ty:tt($($args:expr),*)) => {
        TokenTree::Literal(Literal::$ty($($args),*))
    };
    ($ty:tt($($args:expr),*)) => {
        TokenTree::$ty($ty::new($($args),*))
    };
}

macro_rules! ts {
    ($($tok:tt)+) => {
        vec![$($tok)+].into_iter().collect()
    }
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

pub fn compile_error((start, error): CompileError) -> TokenStream {
    let mut inner = tt!(Literal::string(error));
    inner.set_span(start);
    vec![
        tt!(Ident("compile_error", start)),
        tt!(Punct('!', Alone)),
        tt!(Group(Parenthesis, vec![inner].into_iter().collect())),
    ]
    .into_iter()
    .collect()
}

pub fn guard(writer: TokenStream) -> TokenStream {
    ts!(
        tt!(Ident("let", Span::call_site())),
        tt!(Ident("__writer__", Span::call_site())),
        tt!(Punct(':', Alone)),
        tt!(Punct('&', Alone)),
        tt!(Ident("mut", Span::call_site())),
        tt!(Ident("_", Span::call_site())),
        tt!(Punct('=', Alone)),
        tt!(Punct('$', Alone)),
        tt!(Ident("crate", Span::call_site())),
        tt!(Punct(':', Joint)),
        tt!(Punct(':', Alone)),
        tt!(Group(
            Parenthesis,
            vec![tt!(Punct('&', Alone)), tt!(Ident("mut", Span::call_site())),]
                .into_iter()
                .chain(writer.into_iter())
                .collect()
        )),
        tt!(Punct(';', Alone)),
    )
}

pub fn control(seq: ControlSeq) -> Result<TokenStream, CompileError> {
    unimplemented!();
}
