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
    pub rest: Vec<InputItem>,
}

#[derive(Debug)]
pub enum InputItem {
    Raw(TokenStream),
    Ctrl(ControlSeq),
}

#[derive(Debug)]
pub enum ControlSeq {
    Foreground(TokenStream),
    Background(TokenStream),
    Bold(TokenStream),
    Underline(TokenStream),
    Intense(TokenStream),
    Reset,
}
