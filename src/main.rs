// use rustpython_parser::ast::visitor;
use rustpython_parser::ast::Stmt;
use rustpython_parser::text_size::TextRange;
use rustpython_parser::{ast, Parse};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
            } // Stmt::Assign(assign_statement) => {
            //     imports.push(assign_statement.range);
            //     println!("assign statement value is {:?}", assign_statement.value)
            //     //expr.value
            // }
            // Stmt::Expr(expr) => {
            //     imports.push(expr.range);
            //     println!("expr value is {:?}", expr.value)
            // }
            _ => {
                // println!("Unhandled statement: {:?}", statement);
            }
        }
    }

    imports
}

fn read_file_contents(file_path: &Path) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_python_files<F>(_path: &Path, process_file: &mut F) -> Result<(), std::io::Error>
where
    F: FnMut(&Path, &str) -> Vec<(usize, usize, String)>,
{
    if _path.is_file() && _path.extension().map_or(false, |ext| ext == "py") {
        if let Ok(contents) = read_file_contents(&_path) {
            process_file(&_path, &contents);
        }
        return Ok(());
    } else if _path.is_dir() {
        let entries = fs::read_dir(_path)?;

        for entry in entries {
            let entry = entry?;
            let file_path = entry.path();
            if let Err(err) = read_python_files(&file_path, process_file) {
                eprintln!("Error reading python files: {}", err);
            }
        }
    }

    Ok(())
}

fn parse_source(path: &Path, code: &str) -> Vec<(usize, usize, String)> {
    println!("file {:?}", path);
    let import_statements = find_import_statements(code);
    let mut results = Vec::new();
    for statement in import_statements {
        println!("{:?}", statement.start());
        println!("{:?}", statement.end());
        let start: usize = statement.start().into();
        let end: usize = statement.end().into();
        let snippet = &code[start..end];
        println!("{}", snippet);
        results.push((start, end, snippet.to_string()));
    }
    results
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <path>");
        return;
    }

    let path = Path::new(&args[1]);

    if path.is_file() || path.is_dir() {
        if let Err(err) = read_python_files(&path, &mut parse_source) {
            eprintln!("Error reading python files: {}", err);
        }
    } else {
        eprintln!("Invalid path: {}", path.display());
    }

    // let mut file = File::open("test.py").expect("Failed to open file");

    // // Read the file contents into a string
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)
    //     .expect("Failed to read file");

    // let code: &str = &contents;

    // let code1 = "lst = [1, 2, 3]";
    // let python_expr = ast::Expr::parse(code1, "<test>").unwrap(); // or expr
}
