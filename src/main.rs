use std::env;
use std::fs::File;
use std::io::Read;

use interpreter::interpret_program;
use lexer::Lexer;
use parser::Parser;

pub mod expr;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod statement;
pub mod token;

fn main() {
    // Get file path
    if env::args().len() <= 1 {
        eprintln!("Input error: no input file(s) provided");
        eprintln!(" -h or --help for usage guide and command list\n");
        return;
    }

    let args: Vec<String> = env::args().collect();
    let script_path = args.get(1).unwrap();

    // Check if path is valid
    if !script_path.ends_with(".prot") {
        eprintln!("Input error: unrecognized file extension(s)");
        eprintln!(" please make sure all input files use the '.prot' file extension\n");
        return;
    }

    // Open file
    let mut script: File = {
        let file: File;

        match File::open(script_path) {
            Ok(f) => {
                file = f;
            }

            Err(..) => {
                eprintln!("IO error: could not open file '{}'\n", script_path);
                return;
            }
        }

        file
    };

    // Read file contents into string
    let mut contents = String::new();

    if let Err(..) = script.read_to_string(&mut contents) {
        eprintln!("IO error: failed to read from file '{}'\n", script_path);
        return;
    }

    let mut lexer = Lexer::new(contents);
    let tokens = lexer.collect_tokens();

    if lexer.contains_errors || tokens.is_empty() {
        return;
    }

    let mut parser = Parser::new(tokens);
    let statements = parser.parse_tokens();

    if parser.contains_errors {
        return;
    }

    interpret_program(statements);
}
