use std::collections::HashMap;

use crate::ast::{Expr, Program, Statement, Param};

/// Minimal type model just to get basic checks working.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    I32,
    F64,
    String,
    Void,
    Unknown,
}

fn type_from_name(name: &str) -> Type {
    match name {
        "i32" => Type::I32,
        "f64" => Type::F64,
        "string" => Type::String,
        "void" => Type::Void,
        _ => Type::Unknown,
    }
}

fn type_name(ty: &Type) -> &'static str {
    match ty {
        Type::I32 => "i32",
        Type::F64 => "f64",
        Type::String => "string",
        Type::Void => "void",
        Type::Unknown => "unknown",
    }
}

/// Infer an expression's type from literals and a simple environment (params, locals).
fn infer_expr_type(expr: &Expr, env: &HashMap<String, Type>) -> Type {
    match expr {
        Expr::Number(n) => {
            if n.fract() == 0.0 { Type::I32 } else { Type::F64 }
        }
        Expr::StringLiteral(_) => Type::String,
        Expr::Ident(name) => env.get(name).cloned().unwrap_or(Type::Unknown),
    }
}

/// Build a simple symbol table from parameters (uses declared types when present).
fn build_env(params: &[Param]) -> HashMap<String, Type> {
    let mut env = HashMap::<String, Type>::new();
    for p in params {
        let ty = p.ty.as_deref().map(type_from_name).unwrap_or(Type::Unknown);
        env.insert(p.name.clone(), ty);
    }
    env
}

/// Type-check the program: ensure `return` expressions match the declared return type (if any).
pub fn type_check(program: &Program) -> Result<(), String> {
    for stmt in &program.statements {
        if let Statement::Function { name, params, return_type, body } = stmt {
            let env = build_env(params);
            let expected = return_type
                .as_ref()
                .map(|s| type_from_name(s))
                .unwrap_or(Type::Void);

            for s in body {
                if let Statement::Return(expr) = s {
                    let got = infer_expr_type(expr, &env);
                    if expected != Type::Unknown && expected != Type::Void &&
                       got != Type::Unknown && got != expected {
                        return Err(format!(
                            "Type error in function `{}`: expected `{}` but found `{}`",
                            name, type_name(&expected), type_name(&got)
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

/// Pretty-print the AST to a developer-friendly string (great for debugging).
pub fn pretty(program: &Program) -> String {
    let mut out = String::new();
    for stmt in &program.statements {
        match stmt {
            Statement::Function { name, params, return_type, body } => {
                out.push_str(&format!("fn {}(", name));
                for (i, p) in params.iter().enumerate() {
                    if i > 0 { out.push_str(", "); }
                    if let Some(t) = &p.ty { out.push_str(&format!("{}: {}", p.name, t)); }
                    else { out.push_str(&p.name); }
                }
                out.push(')');
                if let Some(ret) = return_type {
                    out.push_str(&format!(" -> {}", ret));
                }
                out.push_str(" {\n");
                for inner in body {
                    match inner {
                        Statement::Return(expr) => {
                            out.push_str("  return ");
                            match expr {
                                Expr::Number(n) => out.push_str(&format!("{}", n)),
                                Expr::StringLiteral(s) => out.push_str(&format!("\"{}\"", s)),
                                Expr::Ident(id) => out.push_str(id),
                            }
                            out.push_str(";\n");
                        }
                        Statement::Expr(e) => {
                            out.push_str("  ");
                            match e {
                                Expr::Number(n) => out.push_str(&format!("{}", n)),
                                Expr::StringLiteral(s) => out.push_str(&format!("\"{}\"", s)),
                                Expr::Ident(id) => out.push_str(id),
                            }
                            out.push_str(";\n");
                        }
                    }
                }
                out.push_str("}\n\n");
            }
            _ => {}
        }
    }
    out
}

// --------------------- WASM helpers ---------------------

fn write_uleb(mut v: u32, out: &mut Vec<u8>) {
    loop {
        let mut b = (v & 0x7F) as u8;
        v >>= 7;
        if v != 0 { b |= 0x80; }
        out.push(b);
        if v == 0 { break; }
    }
}

fn section(id: u8, content: Vec<u8>, out: &mut Vec<u8>) {
    out.push(id);
    write_uleb(content.len() as u32, out);
    out.extend_from_slice(&content);
}

/// Compile a single exported function where:
///   - return type is `i32`
///   - params are all `i32` (or unspecified; treated as i32 for now)
///   - body is exactly `return <paramIdent>` or `return <int literal>`
/// Exports the function under its Mintora name.
pub fn compile_to_wasm(program: &Program) -> Result<Vec<u8>, String> {
    enum RetSrc { Const(i32), Param(usize) }

    let mut export_name: Option<String> = None;
    let mut param_count: usize = 0;
    let mut param_names: Vec<String> = Vec::new();
    let mut ret_src: Option<RetSrc> = None;

    'search: for stmt in &program.statements {
        if let Statement::Function { name, params, return_type, body } = stmt {
            if return_type.as_deref() != Some("i32") { continue; }
            if body.len() != 1 { continue; }

            // All params must be i32 (or unspecified -> accept as i32 for now)
            let all_i32 = params.iter().all(|p|
                p.ty.as_deref().map(|t| t == "i32").unwrap_or(true)
            );
            if !all_i32 { continue; }

            // Determine return source
            let src = match &body[0] {
                Statement::Return(Expr::Ident(id)) => {
                    if let Some(idx) = params.iter().position(|p| p.name == *id) {
                        RetSrc::Param(idx)
                    } else {
                        continue;
                    }
                }
                Statement::Return(Expr::Number(n)) if n.fract() == 0.0 => {
                    if *n >= 0.0 { RetSrc::Const(*n as i32) } else { continue }
                }
                _ => continue,
            };

            export_name = Some(name.clone());
            param_count = params.len();
            param_names = params.iter().map(|p| p.name.clone()).collect();
            ret_src = Some(src);
            break 'search;
        }
    }

    let export = export_name.ok_or_else(|| {
        "No suitable function found. Expected e.g. `fn <name>(x: i32, ...) -> i32 { return x; }`".to_string()
    })?;
    let ret_src = ret_src.unwrap();

    // ========= Emit WASM =========
    let mut out = Vec::new();
    // header
    out.extend_from_slice(&[0x00, 0x61, 0x73, 0x6D]); // \0asm
    out.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // version 1

    // -- Type section (id=1): one func type (param_count × i32) -> i32
    let mut ty = Vec::new();
    write_uleb(1, &mut ty);      // count
    ty.push(0x60);               // func type
    write_uleb(param_count as u32, &mut ty);
    for _ in 0..param_count { ty.push(0x7F); } // i32 params
    write_uleb(1, &mut ty);      // results = 1
    ty.push(0x7F);               // i32
    section(1, ty, &mut out);

    // -- Function section (id=3): one function that uses type 0
    let mut func = Vec::new();
    write_uleb(1, &mut func);    // count
    write_uleb(0, &mut func);    // type index 0
    section(3, func, &mut out);

    // -- Export section (id=7): export func 0 with Mintora name
    let mut exp = Vec::new();
    write_uleb(1, &mut exp);                         // count
    let name_bytes = export.as_bytes();
    write_uleb(name_bytes.len() as u32, &mut exp);   // name len
    exp.extend_from_slice(name_bytes);
    exp.push(0x00);                                  // kind = func
    write_uleb(0, &mut exp);                         // func index
    section(7, exp, &mut out);

    // -- Code section (id=10): function body
    let mut body = Vec::new();
    body.push(0x00);              // local decls = 0

    match ret_src {
        RetSrc::Const(v) => {
            body.push(0x41);              // i32.const
            write_uleb(v as u32, &mut body);
        }
        RetSrc::Param(idx) => {
            // WASM uses local indices 0..N-1 for function params
            body.push(0x20);              // local.get
            write_uleb(idx as u32, &mut body);
        }
    }
    body.push(0x0B);              // end

    let mut code = Vec::new();
    write_uleb(1, &mut code);     // bodies = 1
    write_uleb(body.len() as u32, &mut code);
    code.extend_from_slice(&body);
    section(10, code, &mut out);

    Ok(out)
}
