// use rustpython_parser::ast::visitor;
use rustpython_parser::ast::{Expr, Stmt};
use rustpython_parser::text_size::TextRange;
use rustpython_parser::{ast, Parse};
use rustpython_parser::{lexer::lex, Mode, Tok};
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::{env, process};

#[derive(Debug, Clone)]
enum StatementType {
    Assign1Tuple,
    AssignTuple,
    AssignList,
    Import,
}

fn handle_expr(expr: &Expr, imports: &mut Vec<(TextRange, StatementType)>) {
    match expr {
        Expr::Tuple(expr_tuple) => {
            if expr_tuple.elts.len() == 1 {
                imports.push((expr_tuple.range, StatementType::Assign1Tuple));
            } else {
                imports.push((expr_tuple.range, StatementType::AssignTuple));
            }
        }
        Expr::List(expr_list) => {
            imports.push((expr_list.range, StatementType::AssignList));
        }
        _ => {}
    }
}

fn find_statements(code: &str, path: &Path) -> Vec<(TextRange, StatementType)> {
    let ast = match ast::Suite::parse(code, "<test>") {
        Ok(parsed_ast) => parsed_ast,
        Err(e) => {
            eprintln!("Failed to parse code: {:?} with error {:?}", path, e);
            // panic!("Parsing error: {:?}", e);
            return [].to_vec();
        }
    };
    let mut statements = Vec::new();

    for statement in ast {
        match statement {
            Stmt::Import(import_statement) => {
                statements.push((import_statement.range, StatementType::Import));
            }
            Stmt::ImportFrom(import_from_statement) => {
                statements.push((import_from_statement.range, StatementType::Import));
            }
            Stmt::Assign(assign_statement) => {
                handle_expr(&assign_statement.value, &mut statements);
            }
            Stmt::Return(_return_statement) => {
                if let Some(return_value) = _return_statement.value {
                    handle_expr(&return_value, &mut statements);
                }
            }
            _ => {}
        }
    }

    statements
}

fn read_file_contents(file_path: &Path) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type FilePath = PathBuf;
type ProcessedData = (usize, usize, String);
type ProcessedFile = Vec<ProcessedData>;
type ProcessedFilesResult = Vec<(FilePath, ProcessedFile)>;

fn read_python_files<F>(
    _path: &Path,
    process_file: &mut F,
) -> Result<ProcessedFilesResult, std::io::Error>
where
    F: FnMut(&Path, &str) -> ProcessedFile,
{
    let mut all_results = Vec::new();
    if _path.is_file() && _path.extension().map_or(false, |ext| ext == "py") {
        if let Ok(contents) = read_file_contents(_path) {
            let results = process_file(_path, &contents);
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
    let _statements = find_statements(code, path);
    let mut results = Vec::new();
    for (statement, _type) in _statements {
        let start: usize = statement.start().into();
        let end: usize = statement.end().into();
        let snippet = &code[start..end];
        match _type {
            StatementType::Import => {
                // println!("{}", snippet);
                let lines: Vec<&str> = snippet.split('\n').collect();
                let has_line_ending_with_backslash =
                    lines.iter().any(|line| line.trim_end().ends_with('\\'));

                if has_line_ending_with_backslash {
                    // println!("Snippet contains a line ending with '\\':\n{}", snippet);
                    results.push((start, end, snippet.to_string()));
                }
            }
            StatementType::Assign1Tuple => {
                if snippet.ends_with(',') {
                    results.push((start, end, snippet.to_string()));
                }
                // println!("{}: {}", _type, snippet);
            }
            StatementType::AssignTuple | StatementType::AssignList => {
                // println!("type {}: {}", _type, snippet);
                let tokens = get_tokens(snippet);
                let mut i = 0;
                while i < tokens.len().saturating_sub(1) {
                    if let (Some((elem1, _range1)), Some((elem2, _range2))) =
                        (tokens.get(i), tokens.get(i + 1))
                    {
                        match (elem1, elem2) {
                            (Tok::String { value: a, .. }, Tok::String { value: b, .. }) => {
                                println!("found adjacent string elements: {:?} and {:?}", a, b);
                                results.push((start, end, snippet.to_string()));
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
    }

    results
}

fn get_tokens(code: &str) -> Vec<(Tok, TextRange)> {
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
                _tokens.push((token, range));
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
        if let Ok(results) = read_python_files(path, &mut parse_source) {
            let mut _len = 0;
            for (_file_path, info) in results {
                for (start, end, snippet) in info {
                    _len += 1;
                    println!(
                        "path {} snippet: {} (from {} to {})",
                        _file_path.display(),
                        snippet,
                        start,
                        end
                    );
                }
            }

            if _len > 0 {
                process::exit(-1)
            }
        }
    } else {
        eprintln!("Invalid path: {}", path.display());
    }
}
