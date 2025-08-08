/// Abstract Syntax Tree for Mintora

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    StringLiteral(String),
    Ident(String),
}

#[derive(Debug)]
pub enum Statement {
    Function {
        name: String,
        params: Vec<String>,
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


