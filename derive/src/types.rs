use proc_macro::{Span, TokenStream};

pub type CompileError = (Span, &'static str);
pub type Result<T> = std::result::Result<T, CompileError>;

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
    Command(String, TokenStream),
    Reset,
}

pub type RawOutput = (String, Vec<TokenStream>);

#[derive(Debug)]
pub enum OutputItem {
    Raw(RawOutput),
    Ctrl(ControlSeq),
}
