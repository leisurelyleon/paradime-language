#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, TokenKind};

    #[test]
    fn lex_basic_sequence() {
        let src = r#"fn foo(x: i32) -> i32 { return 42; }"#;
        let kinds: Vec<_> = Lexer::new(src).map(|t| t.kind).collect();
        assert!(kinds.len() > 0);
        assert!(matches!(kinds[0], TokenKind::Keyword(ref k) if k == "fn"));
    }
}
