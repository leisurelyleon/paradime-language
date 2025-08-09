use crate::ast::{Expr, Param, Program, Statement, BinOp};
use crate::lexer::{Lexer, Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { tokens: lexer.collect(), pos: 0 }
    }

    #[inline] fn at_end(&self) -> bool { self.pos >= self.tokens.len() }
    #[inline] fn peek(&self) -> &Token { &self.tokens[self.pos] }
    #[inline] fn bump(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        self.pos += 1;
        t
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        while !self.at_end() {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            } else { break; }
        }
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Option<Statement>, String> {
        if self.at_end() { return Ok(None); }
        match &self.peek().kind {
            TokenKind::Keyword(k) if k == "fn" => Ok(Some(self.parse_function()?)),
            TokenKind::Keyword(k) if k == "return" => {
                self.bump();
                let expr = self.parse_expression()?;
                self.expect_semicolon()?;
                Ok(Some(Statement::Return(expr)))
            }
            _ => Ok(None),
        }
    }

    fn parse_function(&mut self) -> Result<Statement, String> {
        self.bump(); // fn
        let name = self.expect_ident("function name")?;
        self.expect_symbol('(')?;
        let params = self.parse_params()?;
        self.expect_symbol(')')?;

        let return_type = if let TokenKind::Arrow = &self.peek().kind {
            self.bump();
            Some(self.expect_ident("return type")?)
        } else { None };

        self.expect_symbol('{')?;
        let mut body = Vec::new();
        while !matches!(&self.peek().kind, TokenKind::CloseBrace) {
            if let Some(stmt) = self.parse_statement()? { body.push(stmt); }
            else { return Err(format!("Unexpected token in function body: {:?}", self.peek().kind)); }
        }
        self.expect_symbol('}')?;

        Ok(Statement::Function { name, params, return_type, body })
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, String> {
        let mut params = Vec::new();
        if matches!(&self.peek().kind, TokenKind::CloseParen) { return Ok(params); }
        loop {
            let name = self.expect_ident("parameter name")?;
            let mut ty = None;
            if let TokenKind::Symbol(':') = &self.peek().kind {
                self.bump();
                ty = Some(self.expect_ident("parameter type")?);
            }
            params.push(Param { name, ty });
            if let TokenKind::Symbol(',') = &self.peek().kind { self.bump(); } else { break; }
        }
        Ok(params)
    }

    fn expect_ident(&mut self, what: &str) -> Result<String, String> {
        if self.at_end() { return Err(format!("Expected {} but found <eof>", what)); }
        match self.bump().kind {
            TokenKind::Ident(id) => Ok(id),
            other => Err(format!("Expected {} but found {:?}", what, other)),
        }
    }

    fn expect_symbol(&mut self, sym: char) -> Result<(), String> {
        if self.at_end() { return Err(format!("Expected `{}` but found <eof>", sym)); }
        match &self.peek().kind {
            TokenKind::Symbol(c) if *c == sym => { self.bump(); Ok(()) }
            TokenKind::OpenBrace if sym == '{' => { self.bump(); Ok(()) }
            TokenKind::CloseBrace if sym == '}' => { self.bump(); Ok(()) }
            TokenKind::OpenParen if sym == '(' => { self.bump(); Ok(()) }
            TokenKind::CloseParen if sym == ')' => { self.bump(); Ok(()) }
            other => Err(format!("Expected symbol `{}` but found {:?}", sym, other)),
        }
    }

    fn expect_semicolon(&mut self) -> Result<(), String> {
        if self.at_end() { return Err("Expected `;` but found <eof>".into()); }
        match &self.peek().kind {
            TokenKind::Semicolon => { self.bump(); Ok(()) }
            other => Err(format!("Expected `;` but found {:?}", other)),
        }
    }

    // -------- expressions (support +) --------

    fn parse_expression(&mut self) -> Result<Expr, String> {
        let left = self.parse_primary()?;
        self.parse_binop_rhs(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        if self.at_end() { return Err("Unexpected end of input in expression".into()); }
        match &self.peek().kind {
            TokenKind::Number(n) => { let v: f64 = n.parse().map_err(|_| "Invalid number")?; self.bump(); Ok(Expr::Number(v)) }
            TokenKind::StringLiteral(s) => { let lit = s.clone(); self.bump(); Ok(Expr::StringLiteral(lit)) }
            TokenKind::Ident(id) => { let name = id.clone(); self.bump(); Ok(Expr::Ident(name)) }
            other => Err(format!("Unexpected token in expression: {:?}", other)),
        }
    }

    fn parse_binop_rhs(&mut self, mut left: Expr) -> Result<Expr, String> {
        loop {
            if self.at_end() { break; }
            match &self.peek().kind {
                TokenKind::Symbol('+') => {
                    self.bump();
                    let right = self.parse_primary()?;
                    left = Expr::Binary { op: BinOp::Add, left: Box::new(left), right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(left)
    }
}
