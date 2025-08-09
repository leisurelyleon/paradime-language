use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Keyword(String),
    Ident(String),
    Number(String),
    StringLiteral(String),
    Symbol(char),
    Arrow,      // '->'
    OpenBrace,  // '{'
    CloseBrace, // '}'
    OpenParen,  // '('
    CloseParen, // ')'
    Semicolon,  // ';'
    Eof, 
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: (usize, usize), // (start, end)
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    idx: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { input: src.chars().peekable(), idx: 0 }
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.input.next()?;
        self.idx += c.len_utf8();
        Some(c)
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    pub fn next_token(&mut self) -> Token {
        // skip whitespace
        while let Some(&c) = self.peek() {
            if c.is_whitespace() { self.bump(); } else { break; }
        }
        
        let start = self.idx;
        let kind = match self.bump() {
            Some('/') if self.peek() == Some(&'/') => {
                // line comment
                while let Some(&c) = self.peek() {
                    if c == '\n' { break; }
                    self.bump();
                }
                return self.next_token();
            }
            Some('"') => {
                let mut s = String::new();
                while let Some(&c) = self.peek() {
                    if c == '"' { self.bump(); break; }
                    s.push(self.bump().unwrap());
                }
                TokenKind::StringLiteral(s)
            }
            Some(c) if c.is_ascii_digit() => {
                let mut num = c.to_string();
                while let Some(&d) = self.peek() {
                    if d.is_ascii_digit() || d == '.' {
                        num.push(self.bump().unwrap());
                    } else { break; }
                }
                TokenKind::Number(num)
            }
            Some(c) if c.is_alphabetic() || c == '_' => {
                let mut ident = c.to_string();
                while let Some(&d) = self.peel() {
                    if d.is_alphanumeric() || d == '_' {
                        ident.push(self.bump().unwrap());
                    } else { break; }
                }
                match ident.as_str() {
                    "contract" | "fn" | "return" | "if" | "else" =>
                        TokenKind::Keyword(ident),
                    _=> TokenKind::Ident(ident),
                }
            }
            Some('_') if self.peek() == Some(&'>') => { self.bump(); TokenKind::Arrow }
            Some('{') => TokenKind::OpenBrace,
            Some('}') => TokenKind::CloseBrace,
            Some('(') => TokenKind::OpenParen,
            Some(')') => TokenKind::CloseParen,
            Some(';') => TokenKind::Semicolon,
            Some(c) => TokenKind::Symbol(c),
            None => TokenKind::Eof,
        };
        
        let end = self.idx;
        Token { kind, span: (start, end) }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.next_token();
        if t.kind == TokenKind::Eof { None } else { Some(t) }
    }
}
