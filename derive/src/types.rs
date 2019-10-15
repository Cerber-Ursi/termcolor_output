use proc_macro::{Span, TokenStream};

pub type CompileError = (Span, &'static str);

#[derive(Debug)]
pub enum FormatPart {
    Text(String),
    Input(String),
}

#[derive(Debug)]
pub struct FormatItems {
    pub span: Span,
    pub parts: Vec<FormatPart>,
}

#[derive(Debug)]
pub struct MacroInput {
    pub writer: TokenStream,
    pub format: FormatItems,
    pub rest: Vec<TokenStream>,
}
