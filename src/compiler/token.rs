#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(String),
    Identifier(String),
    String(String),
    True,
    False,

    LeftParen,
    RightParen,

    // EOF
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}
