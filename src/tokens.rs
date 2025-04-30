#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Identifier,
    String,

    OParen,
    CParen,

    Comma,
    Semicolon,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub location: Location,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub start_col: usize,
    pub end_col: usize,

    pub start_line: usize,
    pub end_line: usize,

    pub file_path: String,
    pub line: String,
}
