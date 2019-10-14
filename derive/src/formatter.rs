#[derive(Debug, Clone)]
pub enum FormatterKind {
    Debug,
    Display,
    Unknown(String),
}

pub enum FormatterItem<'a> {
    Text(&'a str),
    Format(FormatterKind),
}

#[derive(PartialEq, Eq)]
enum ScanState {
    Text(String),
    Opened,
    Colon,
    Hash,
}

// pub fn parse<'a>(&'a str) -> Vec<FormatterItem<'a>> {

// }
