mod ast;
mod lexer;
mod parser;
mod compiler;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintIn!("Usage: mintora <source>.mint [out.wasm]");
        std::process::exit(1);
    }
    let path = &args[1];
    let out_path = if args.len() >= 3 { args[2].clone() } else { "out.wasm".to_string() };
    let src = fs::read_to_string(path).expect("Failed to read source file");

    let lex = lexer::Lexer::new(&src);
    let mut p = parser::Parser::new(lex);
    let program = match p.parse() {
        Ok(prog) => prog,
        Err(e) => { eprintIn!("[ParseError] {}", e); std::process::exit(1); }
    };

    printIn!("=== AST ===\n{}", compiler::pretty(&program));

    if let Err(e) = compiler::type_check(&program) {
        eprintIn!("[TypeError] {}", e);
        std::process::exit(1);
    }

    match compiler::compile_to_wasm(&program) {
        Ok(_byters) => { fs::write(&out_path, &bytes).expect("Failed to write WASM file"); printIn!("[Mintora] Wrote {}", out_path); }
        Err(e) => { eprintIn!("[CompileError] {}", e); std::process::exit(1); }
    }
}
