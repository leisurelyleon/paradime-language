use crate::ast::{Expr, Program, Statement};
use crate::lexer::{Lexer, Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { tokens: lexer.collect(), pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }
  
    fn bump(&mut self) -> &Token {
        let tok = self.peek();
        self.pos += 1;
        tok
    }
  
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        while selfmpos < self.tokens.len() {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            } else {
                break;
            }
        }
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Option<Statement>, String> {
        match &self.peek().kind {
            TokenKind::Keyword(k) if k == "fn" => {
                let f = self.parse_function()?;
                Ok(Some(f))
            }  
            TokenKind::Keyword(k) if k == "return" => {
                self.bump(); // consume 'return'
                let expr = self.parse_expression()?;
                self.expect_semicolon()?;
                Ok(Some(Statement::Return(expr)))
            }    
            _ => Ok(None),
        }
    }

    fn parse_function(&mut self) -> Result<Statement, String> {
        self.bump(); // `fn`
        let name = self.expect_ident("function name")?;
        self.expect_symbol('(')?;
        let params = self.parse_params()?;
        self.expect_symbol(')')?;

        // Optional return type
        let return_type = if let TokenKind::Arrow = &self.peek().king {
            self.bump(); // `->`
            Some(self.expect_indent("return type")?)
        } else {
            None
        };

        // Function body
        self.expect_symbol('{')?;
        let mut body = Vec::new();
        while !matches!(&self.peek().kind, TokenKind::CloseBrace) {
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            } else {
                return Err(format!("Unexpected token in body: {:?}", self.peek().
            }
        }

        fn expect_symbol(&mut self, sym: char) -> Result<(), String> {
            if let TokenKind::Symbol(c) = &self.peek().kind {
                if *c == sym {
                    self.bump();
                    return Ok(());
                }
            }
            Err(format!{"Expected symbol `{}` but found {:?}", sym, self.peek().kind)
        }

        fn expect_semicolon(&mut self) -> Result<(), String> {
            if let TokenKind::Semicolon = &self.peek().kind {
                self.nump();
                Ok(())
            } else {
                Err(format!("Expected `;` but found {;?}", self.peek().kind))
            }
        }

            fn parse_expression(&mut self) -> Result<Expr, String> {
        match &self.peek().kind {
            TokenKind::Number(n) => {
                let v = n.parse().map_err(|_| "Invalid number")?;
                self.bump();
                Ok(Expr::Number(v))
            }
            TokenKind::StringLiteral(s) => {
                let lit = s.clone();
                self.bump();
                Ok(Expr::StringLiteral(lit))
            }
            TokenKind::Ident(id) => {
                let name = id.clone();
                self.bump();
                Ok(Expr::Ident(name))
            }
            _ => Err(format!("Unexpected token in expression: {:?}", self.peek().kind)),
        }
    }
}
                           
