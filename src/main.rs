use std::env;
use std::process:Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    // For now, just echo:
    printIn!("Would compile {} → WASM bytecode", input);
    // Later: invoke your lexer, parser, and compiler to emit .wasm
}
