use crate::ast::{Expr, Param, Program, Statement};
use crate::lexer::{Lexer, Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { tokens: lexer.collect(), pos: 0 }
    }

    fn peek(&self) -> &Token { &self.tokens[self.pos] }
    fn bump(&mut self) -> &Token { let t = self.peek(); self.pos += 1; t }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        while self.pos < self.tokens.len() {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            } else { break; }
        }
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Option<Statement>, String> {
        match &self.peek().kind {
            TokenKind::Keyword(k) if k == "fn" => Ok(Some(self.parse_function()?)),
            TokenKind::Keyword(k) if k == "return" => {
                self.bump(); // 'return'
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

        // Optional return type: -> Type
        let return_type = if let TokenKind::Arrow = &self.peek().kind {
            self.bump();
            Some(self.expect_ident("return type")?)
        } else { None };

        self.expect_symbol('{')?;
        let mut body = Vec::new();
        while !matches!(&self.peek().kind, TokenKind::CloseBrace) {
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            } else {
                return Err(format!("Unexpected token in body: {:?}", self.peek().kind));
            }
        }
        self.expect_symbol('}')?;

        Ok(Statement::Function { name, params, return_type, body })
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, String> {
        let mut params = Vec::new();
        if matches!(&self.peek().kind, TokenKind::CloseParen) {
            return Ok(params);
        }
        loop {
            let name = self.expect_ident("parameter name")?;
            let mut ty = None;
            if let TokenKind::Symbol(':') = &self.peek().kind {
                self.bump(); // ':'
                ty = Some(self.expect_ident("parameter type")?);
            }
            params.push(Param { name, ty });

            if let TokenKind::Symbol(',') = &self.peek().kind {
                self.bump(); // continue
            } else {
                break;
            }
        }
        Ok(params)
    }

    fn expect_ident(&mut self, ctx: &str) -> Result<String, String> {
        if let TokenKind::Ident(id) = &self.bump().kind { Ok(id.clone()) }
        else { Err(format!("Expected {} but found {:?}", ctx, self.peek().kind)) }
    }

    fn expect_symbol(&mut self, sym: char) -> Result<(), String> {
        if let TokenKind::Symbol(c) = &self.peek().kind {
            if *c == sym { self.bump(); return Ok(()); }
        }
        Err(format!("Expected symbol `{}` but found {:?}", sym, self.peek().kind))
    }

    fn expect_semicolon(&mut self) -> Result<(), String> {
        if let TokenKind::Semicolon = &self.peek().kind { self.bump(); Ok(()) }
        else { Err(format!("Expected `;` but found {:?}", self.peek().kind)) }
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        match &self.peek().kind {
            TokenKind::Number(n) => { let v = n.parse().map_err(|_| "Invalid number")?; self.bump(); Ok(Expr::Number(v)) }
            TokenKind::StringLiteral(s) => { let lit = s.clone(); self.bump(); Ok(Expr::StringLiteral(lit)) }
            TokenKind::Ident(id) => { let name = id.clone(); self.bump(); Ok(Expr::Ident(name)) }
            _ => Err(format!("Unexpected token in expression: {:?}", self.peek().kind)),
        }
    }
}
