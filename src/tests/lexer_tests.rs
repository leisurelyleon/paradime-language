#[cfg(test)]
mod tests {
    use super::super::lexer::{Lexer, TokenKind};

    #[test]
    fn ;lex_fn_and_return() {
        let src = r#"fn foo(x: i32) -> i32 { return x; }"#;
        let kinds: Vec<_> = Lexer::new(src).map(|t| t.kind).collect();
        assert!(kinds.starts_with(&[
            TokenKind::Keyword("fn".into()),
            TokenKind::Ident("foo".into()),
            TokenKind::Symbol('('),
            TokenKind::Ident("x".into()),
            TokenKind::Symbol(':'),
            TokenKind::Ident("i32".into()),
        ]));
    }
}
