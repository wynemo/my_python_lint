// use rustpython_parser::ast::visitor;
use rustpython_parser::ast::{Expr, Stmt};
use rustpython_parser::text_size::TextRange;
use rustpython_parser::{ast, Parse};
use rustpython_parser::{lexer::lex, Mode, StringKind, Tok};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn find_statements(code: &str) -> Vec<(TextRange, String)> {
    let ast = ast::Suite::parse(code, "<test>").unwrap();
    let mut imports = Vec::new();

    for statement in ast {
        match statement {
            Stmt::Import(import_statement) => {
                imports.push((import_statement.range, "import".to_string()));
            }
            Stmt::ImportFrom(import_from_statement) => {
                imports.push((import_from_statement.range, "import".to_string()));
            }
            Stmt::Assign(assign_statement) => {
                match *assign_statement.value {
                    Expr::Tuple(expr_tuple) => {
                        // println!("tuple expr elts is {:?}", expr_tuple.elts);
                        if expr_tuple.elts.len() == 1 {
                            imports.push((expr_tuple.range, "assign".to_string()));
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
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

fn read_python_files<F>(
    _path: &Path,
    process_file: &mut F,
) -> Result<Vec<(PathBuf, Vec<(usize, usize, String)>)>, std::io::Error>
where
    F: FnMut(&Path, &str) -> Vec<(usize, usize, String)>,
{
    let mut all_results = Vec::new();
    if _path.is_file() && _path.extension().map_or(false, |ext| ext == "py") {
        if let Ok(contents) = read_file_contents(&_path) {
            let results = process_file(&_path, &contents);
            all_results.push((_path.to_path_buf(), results));
        }
        return Ok(all_results);
    } else if _path.is_dir() {
        let entries = fs::read_dir(_path)?;

        for entry in entries {
            let entry = entry?;
            let file_path = entry.path();
            if let Ok(results) = read_python_files(&file_path, process_file) {
                all_results.extend(results);
            } else {
                //eprintln!("Error reading python files: {}", err);
                eprintln!("Error reading python files in directory: {:?}", file_path);
            }
        }
    }

    Ok(all_results)
}

fn parse_source(path: &Path, code: &str) -> Vec<(usize, usize, String)> {
    // println!("file {:?}", path);
    let _statements = find_statements(code);
    let mut results = Vec::new();
    for (statement, _type) in _statements {
        let start: usize = statement.start().into();
        let end: usize = statement.end().into();
        let snippet = &code[start..end];
        if _type == "import" {
            // println!("{}", snippet);
            let lines: Vec<&str> = snippet.split('\n').collect();
            let has_line_ending_with_backslash =
                lines.iter().any(|line| line.trim_end().ends_with('\\'));

            if has_line_ending_with_backslash {
                // println!("Snippet contains a line ending with '\\':\n{}", snippet);
                results.push((start, end, snippet.to_string()));
            }
        } else if _type == "assign" {
            if snippet.ends_with(",") {
                results.push((start, end, snippet.to_string()));
            }
            // println!("{}: {}", _type, snippet);
        }
    }
    results
}

fn get_tokens(code: &str) -> Vec<Tok> {
    let tokens = lex(code, Mode::Module)
        .map(|tok| tok.expect("Failed to lex"))
        .collect::<Vec<_>>();
    let mut _tokens = Vec::new();

    for (token, range) in tokens {
        match token {
            Tok::NonLogicalNewline => continue,
            Tok::Newline => continue,
            Tok::Indent => continue,
            Tok::Dedent => continue,
            _ => {
                // println!("{token:?}@{range:?}",);
                _tokens.push(token);
            }
        }
    }
    _tokens
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <path>");
        return;
    }

    let path = Path::new(&args[1]);

    if path.is_file() || path.is_dir() {
        if let Ok(results) = read_python_files(&path, &mut parse_source) {
            // eprintln!("Error reading python files: {}", err);
            for (_file_path, imports) in results {
                // println!("File: {:?}", file_path);
                for (start, end, snippet) in imports {
                    println!(
                        "path {} statement: {} (from {} to {})",
                        _file_path.display(),
                        snippet,
                        start,
                        end
                    );
                }
            }
        }
        if path.is_file() {
            if let Ok(contents) = read_file_contents(&path) {
                let tokens = get_tokens(&contents);
                let mut i = 0;
                while i < tokens.len() - 1 {
                    if let (Some(elem1), Some(elem2)) = (tokens.get(i), tokens.get(i + 1)) {
                        match (elem1, elem2) {
                            (Tok::String { value: a, .. }, Tok::String { value: b, .. }) => {
                                println!("Found adjacent TypeA elements: {:?} and {:?}", a, b);
                                i += 2; // Skip the next element
                            }
                            _ => {
                                i += 1;
                            }
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Invalid path: {}", path.display());
    }
}
