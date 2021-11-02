use super::token::{Token, TokenKind};

pub struct Lexer {
    source: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: Vec<char>) -> Lexer {
        Lexer {
            source,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    fn get_current(&self) -> Option<char> {
        if self.is_past_end() {
            return None;
        }
        Some(self.source[self.current])
    }

    fn is_past_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn skip_whitespace(&mut self) {
        while !self.is_past_end() {
            match self.get_current().unwrap() {
                ' ' | '\t' => {
                    self.current += 1;
                }

                '\n' | '\r' => {
                    self.column = 1;
                    self.current += 1;
                    self.line += 1;
                }

                _ => {
                    break;
                }
            }
        }
    }

    fn new_token(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            line: self.line,
            column: self.column,
        }
    }

    /// Advances `Lexer.current` and returns the new current char.
    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.column += 1;
        self.get_current()
    }

    /// "Chomps" on input and returns a new token.
    fn chomp(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespace();
        if self.is_past_end() {
            return Ok(None);
        }

        match self.get_current().unwrap() {
            // one- and two-char tokens
            '(' => {
                self.advance();
                Ok(Some(self.new_token(TokenKind::LeftParen)))
            }
            ')' => {
                self.advance();
                Ok(Some(self.new_token(TokenKind::RightParen)))
            }
            '=' => {
                self.advance();
                Ok(Some(
                    self.new_token(TokenKind::Identifier(String::from('='))),
                ))
            }
            '+' => {
                self.advance();
                Ok(Some(
                    self.new_token(TokenKind::Identifier(String::from('+'))),
                ))
            }
            '-' => {
                self.advance();
                Ok(Some(
                    self.new_token(TokenKind::Identifier(String::from('-'))),
                ))
            }
            '*' => {
                self.advance();
                Ok(Some(
                    self.new_token(TokenKind::Identifier(String::from('*'))),
                ))
            }
            '/' => {
                self.advance();
                Ok(Some(
                    self.new_token(TokenKind::Identifier(String::from('/'))),
                ))
            }

            // strings
            '"' => {
                let mut lexeme = String::new();
                while let Some(i) = self.advance() {
                    if i == '"' {
                        self.advance();
                        break;
                    }
                    lexeme.push(i);
                }
                if self.is_past_end() {
                    return Err(String::from("Unterminated string"));
                }

                Ok(Some(self.new_token(TokenKind::String(lexeme))))
            }

            // numbers
            ch if ch.is_digit(10) => {
                let mut is_float = false;
                let mut lexeme = String::from(ch);
                while let Some(i) = self.advance() {
                    match i {
                        '.' => {
                            if !is_float {
                                is_float = true;
                                lexeme.push(i);
                            } else {
                                return Err(String::from(
                                    "Invalid floating-point number signature",
                                ));
                            }
                        }
                        '0'..='9' => {
                            lexeme.push(i);
                        }
                        _ => break,
                    }
                }
                Ok(Some(self.new_token(TokenKind::Number(lexeme))))
            }

            // identifiers
            ch if ch.is_ascii_alphabetic() => {
                let mut lexeme = String::from(ch);

                while let Some(i) = self.advance() {
                    if !i.is_ascii_alphanumeric() {
                        break;
                    }
                    lexeme.push(i);
                }
                let kind: TokenKind;
                match lexeme.as_str() {
                    "true" => kind = TokenKind::True,
                    "false" => kind = TokenKind::False,
                    // "let" =>    kind = TokenKind::LetKeyword,
                    // "leave" =>  kind = TokenKind::LeaveKeyword,
                    // "but" =>    kind = TokenKind::ButKeyword,
                    // "repeat" => kind = TokenKind::RepeatKeyword,
                    _ => kind = TokenKind::Identifier(lexeme),
                }
                Ok(Some(self.new_token(kind)))
            }

            ch => {
                let mut m = String::from("Unrecognized character ");
                m.push(ch);
                Err(m)
            }
        }
    }

    pub fn run(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.is_past_end() {
            match self.chomp() {
                Err(e) => return Err(e),
                Ok(Some(t)) => tokens.push(t),
                Ok(None) => break,
            }
        }

        tokens.push(self.new_token(TokenKind::EOF));

        Ok(tokens)
    }
}
