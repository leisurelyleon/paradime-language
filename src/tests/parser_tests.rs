#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser, ast::{Statement, Param}};

    #[test]
    fn parse_params_and_return_type() {
        let src = "fn add(a: i32, b: i32) -> i32 { return a; }";
        let mut p = Parser::new(Lexer::new(src));
        let prog = p.parse().expect("Failed to parse function");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::Function { name, params, return_type, body } => {
                assert_eq!(name, "add");
                assert_eq!(params.len(), 2);
                assert_eq!(params[0].name, "a");
                assert_eq!(params[0].ty.as_deref(), Some("i32"));
                assert_eq!(params[1].name, "b");
                assert_eq!(params[1].ty.as_deref(), Some("i32"));
                assert_eq!(return_type.as_deref(), Some("i32"));
                assert_eq!(body.len(), 1);
            }
            _ => panic!("Expected function statement"),
        }
    }
}
