// use rustpython_parser::ast::visitor;
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
            Stmt::Assign(assign_statement) => {
                imports.push(assign_statement.range);
                println!("assign value is {:?}", assign_statement.value)
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
    let code = "(1, 2,)\nt=((1, 2), 3)\nf((1, 1,), 2)\nfrom objects.asset import AssetTask, AssetAccountCheck, AssetCheck, AssetAccountScan, AssetScan, Asset, \\\n    AssetAccountScanRelation, Account, DEFAULT_VPC_NAME, Tag\nl = 1\nl += 1";

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
