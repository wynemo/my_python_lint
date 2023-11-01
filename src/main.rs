use rustpython_parser::ast::Stmt;
use rustpython_parser::text_size::TextRange;
use rustpython_parser::{ast, Parse};

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
            _ => {}
        }
    }

    imports
}

fn main() {
    let code = "from objects.asset import AssetTask, AssetAccountCheck, AssetCheck, AssetAccountScan, AssetScan, Asset, \\\n    AssetAccountScanRelation, Account, DEFAULT_VPC_NAME, Tag";

    let import_statements = find_import_statements(code);
    for statement in import_statements {
        println!("{:?}", statement.start());
        println!("{:?}", statement.end());
        let _start: usize = statement.start().into();
        let _end: usize = statement.end().into();
        println!("{}", &code[_start.._end])
    }
}
