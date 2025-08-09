/// Abstract Syntax Tree for Mintora

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Option<String>, // e.g., "i32", "string"
}

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    StringLiteral(String),
    Ident(String),
    Binary { op: BinOp, left: Box<Expr>, right: Box<Expr> },
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp { Add }

#[derive(Debug)]
pub enum Statement {
    Function {
        name: String,
        params: Vec<Param>,
        return_type: Option<String>,
        body: Vec<Statement>,
    },
    Return(Expr),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
