use proc_macro::{Span, TokenStream};

pub type CompileError = (Span, &'static str);

pub enum FormatPart {
    Text(String),
    Input(String),
}

pub struct FormatItems {
    pub span: Span,
    pub parts: Vec<FormatPart>,
}

pub struct MacroInput {
    pub writer: TokenStream,
    pub format: FormatItems,
    pub rest: Vec<TokenStream>,
}
