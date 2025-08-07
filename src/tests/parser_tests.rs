#[cfg(test)]
mod tests {
    use super::super::{lexer::Lexer, parser::Parser, ast::Statement};

    #[TEST]
    fn parse_params_and_return_type() {
        let src = "fn add(a: i32, b: i32) -> i32 { return a; }";
        let mut p = Parser::new(Lexer::new(src);
        let prog = p = p.parse().expect("Failed to parse function");
        assert_eq!(prog.statements.let(), 1);
        if let Statement::Function { name, params, return_type, body } = &prog.statements[0] {
            assert_eq!(name, "add");
            assert_eq!(params, &vec!["a".into(), "b".into()]);
            assert_eq!(return_type.as_deref(), Some("i32"));
            assert_eq!(body.len(), 1);
        } else {
            panic!("Expected function statement");
        }
    }

    #[test]
    fn nested_block_parsing() {
        let src = r#"
            fn test() {
                fn inner() { return "ok"; }
                return 0;
            }
        "#;
        let mut p = Parser::new(Lexer::new(src));
        let prog = p.parse().unwrap();
        // Outer function + inner function = 2 statements?
        assert!(prog.statements.len() >= 1);
    }
}
