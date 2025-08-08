use std::collections::HashMap;

use crate::ast::{Expr, Param, Program, Statement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type { I32, F64, String, Void, Unknown }

fn type_from_name(name: &str) -> Type {
    match name {
        "i32" => Type::I32,
        "f64" => Type::F64,
        "string" => Type::String,
        "void" => Type::Void,
        _ => Type::Unknown,
    }
}
fn type_name(t: &Type) -> &'static str {
    match t { Type::I32=>"i32", Type::F64=>"f64", Type::String=>"string", Type::Void=>"void", Type::Unknown=>"unknown" }
}

/// Infer an expression type from literals and environment.
fn infer_expr_type(expr: &Expr, env: &HashMap<String, Type>) -> Type {
    match expr {
        Expr::Number(n) => if n.fract()==0.0 { Type::I32 } else { Type::F64 },
        Expr::StringLiteral(_) => Type::String,
        Expr::Ident(name) => env.get(name).cloned().unwrap_or(Type::Unknown),
    }
}

/// Build a simple symbol table from parameters (uses declared types when present).
fn build_env(params: &[Param]) -> HashMap<String, Type> {
    let mut env = HashMap::new();
    for p in params {
        let ty = p.ty.as_deref().map(type_from_name).unwrap_or(Type::Unknown);
        env.insert(p.name.clone(), ty);
    }
    env
}

pub fn type_check(program: &Program) -> Result<(), String> {
    for stmt in &program.statements {
        if let Statement::Function { name, params, return_type, body } = stmt {
            let env = build_env(params);
            let expected = return_type.as_deref().map(type_from_name).unwrap_or(Type::Void);

            for s in body {
                if let Statement::Return(expr) = s {
                    let got = infer_expr_type(expr, &env);
                    // Only flag obvious mismatches (ignore Unknown for now)
                    if expected != Type::Void && got != Type::Unknown && got != expected {
                        return Err(format!(
                            "Type error in `{}`: expected `{}` but found `{}`",
                            name, type_name(&expected), type_name(&got)
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn pretty(program: &Program) -> String {
    let mut out = String::new();
    for stmt in &program.statements {
        if let Statement::Function { name, params, return_type, body } = stmt {
            out.push_str(&format!("fn {}(", name));
            for (i, p) in params.iter().enumerate() {
                if i>0 { out.push_str(", "); }
                if let Some(t) = &p.ty { out.push_str(&format!("{}: {}", p.name, t)); }
                else { out.push_str(&p.name); }
            }
            out.push(')');
            if let Some(ret) = return_type { out.push_str(&format!(" -> {}", ret)); }
            out.push_str(" {\n");
            for s in body {
                match s {
                    Statement::Return(e) => {
                        out.push_str("  return ");
                        match e {
                            Expr::Number(n)=> out.push_str(&n.to_string()),
                            Expr::StringLiteral(s)=> out.push_str(&format!("\"{}\"", s)),
                            Expr::Ident(id)=> out.push_str(id),
                        }
                        out.push_str(";\n");
                    }
                    Statement::Expr(e) => {
                        out.push_str("  ");
                        match e {
                            Expr::Number(n)=> out.push_str(&n.to_string()),
                            Expr::StringLiteral(s)=> out.push_str(&format!("\"{}\"", s)),
                            Expr::Ident(id)=> out.push_str(id),
                        }
                        out.push_str(";\n");
                    }
                }
            }
            out.push_str("}\n\n");
        }
    }
    out
}

pub fn compile_to_wasm(_program: &Program) -> Result<Vec<u8>, String> {
    // TODO: integrate real WASM emission (e.g., wasm-encoder)
    Ok(Vec::new())
}
