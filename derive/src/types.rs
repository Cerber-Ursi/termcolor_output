use proc_macro::{Span, TokenStream};

pub type CompileError = (Span, &'static str);

pub struct MacroInput {
    pub writer: TokenStream,
    pub format: String,
    pub rest: Vec<TokenStream>,
}
