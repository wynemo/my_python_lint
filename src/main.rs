// use rustpython_parser::ast::visitor;
use rustpython_parser::ast::Stmt;
use rustpython_parser::text_size::TextRange;
use rustpython_parser::{ast, Parse};
use std::fs::File;
use std::io::prelude::*;

fn find_import_statements(code: &str) -> Vec<TextRange> {
    let ast = ast::Suite::parse(code, "<test>").unwrap();
    let mut imports = Vec::new();

    for statement in ast {
        match statement {
            Stmt::Import(import_statement) => {
                imports.push(import_statement.range);
            }
            Stmt::ImportFrom(import_from_statement) => {
                imports.push(import_from_statement.range);
            }
            Stmt::Assign(assign_statement) => {
                imports.push(assign_statement.range);
                println!("assign statement value is {:?}", assign_statement.value)
                //expr.value
            }
            Stmt::Expr(expr) => {
                imports.push(expr.range);
                println!("expr value is {:?}", expr.value)
            }
            _ => {
                println!("Unhandled statement: {:?}", statement);
            }
        }
    }

    imports
}

fn main() {
    let mut file = File::open("test.py").expect("Failed to open file");

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let code: &str = &contents;
    let import_statements = find_import_statements(code);
    for statement in import_statements {
        println!("{:?}", statement.start());
        println!("{:?}", statement.end());
        let _start: usize = statement.start().into();
        let _end: usize = statement.end().into();
        println!("{}", &code[_start.._end])
    }

    // let code1 = "lst = [1, 2, 3]";
    // let python_expr = ast::Expr::parse(code1, "<test>").unwrap(); // or expr
}
