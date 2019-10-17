use proc_macro::{
    Delimiter::*, Group, Ident, Literal, Punct, Spacing::*, Span, TokenStream, TokenTree,
};

use crate::*;

macro_rules! tt {
    (Literal::$ty:tt($($args:expr),*)) => {
        TokenTree::Literal(Literal::$ty($($args),*))
    };
    ($ty:tt($($args:expr),*)) => {
        TokenTree::$ty($ty::new($($args),*))
    };
}

macro_rules! ts {
    () => { TokenStream::new() };
    (let $let:ident = $($tok:tt)+) => {
        let $let: TokenStream = ts!($($tok)+);
    };
    ($($tok:tt)+) => {
        vec![$($tok)+].into_iter().collect()
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
        tt!(Ident("guard", Span::call_site())),
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

pub fn spec_init() -> TokenStream {
    ts!(
        tt!(Ident("let", Span::call_site())),
        tt!(Ident("mut", Span::call_site())),
        tt!(Ident("__spec__", Span::call_site())),
        tt!(Punct('=', Alone)),
        tt!(Ident("termcolor", Span::call_site())),
        tt!(Punct(':', Joint)),
        tt!(Punct(':', Alone)),
        tt!(Ident("ColorSpec", Span::call_site())),
        tt!(Punct(':', Joint)),
        tt!(Punct(':', Alone)),
        tt!(Ident("new", Span::call_site())),
        tt!(Group(Parenthesis, ts!())),
        tt!(Punct(';', Alone))
    )
}

pub fn output(entry: OutputItem) -> Result<TokenStream> {
    match entry {
        OutputItem::Ctrl(seq) => Ok(control(seq)),
        OutputItem::Raw(entry) => raw(entry),
    }
}

fn raw(entry: RawOutput) -> Result<TokenStream> {
    eprintln!("{:?}", entry);
    unimplemented!();
}

fn control(seq: ControlSeq) -> TokenStream {
    ts!(let head =
        tt!(Ident("__spec__", Span::call_site())),
        tt!(Punct('.', Alone)),
    );
    let change_spec: TokenStream = match seq {
        ControlSeq::Reset => ts!(
            tt!(Ident("clear", Span::call_site())),
            tt!(Group(Parenthesis, ts!())),
            tt!(Punct(';', Alone)),
        ),
        ControlSeq::Command(cmd, inner) => ts!(
            tt!(Ident(&(String::from("set_") + &cmd), Span::call_site())),
            tt!(Group(Parenthesis, inner)),
            tt!(Punct(';', Alone)),
        ),
    };
    ts!(let set_spec =
        tt!(Ident("__writer__", Span::call_site())),
        tt!(Punct('.', Alone)),
        tt!(Ident("set_color", Span::call_site())),
        tt!(Group(Parenthesis, ts!(
            tt!(Punct('&', Alone)),
            tt!(Ident("__spec__", Span::call_site()))
        ))),
        tt!(Punct(';', Alone)),
    );
    head.into_iter()
        .chain(change_spec.into_iter())
        .chain(set_spec.into_iter())
        .collect()
}
