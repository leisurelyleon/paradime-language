#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser, ast::Statement};

    #[test]
    fn parse_fn_with_return() {
        let src = "fn add(a: i32, b: i32) -> i32 { return a; }";
        let mut p = Parser::new(Lexer::new(src));
        let prog = p.parse().expect("parse failed");
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::Function { name, params, return_type, body } => {
                assert_eq!(name, "add");
                assert_eq!(params, &vec!["a".into(), "b".into()]);
                assert_eq!(return_type.as_deref(), Some("i32"));
                assert!(!body.is_empty());
            }
            _ => panic!("expected function"),
        }
    }
}
