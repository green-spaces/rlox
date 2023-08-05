// pub mod ast;
// pub mod ast_struct_macros;
// pub mod ast_visitor;
mod scanner;
mod token;
// pub mod parser;
mod ast_enum;
pub mod enum_parser;
mod enum_stmt;
mod environment;
pub mod error;
mod interpreter;
// mod reverse_polish_notation_visitor;

use std::{
    fs,
    io::{self, Write},
    process,
};

pub use error::{RunTimeError, SyntaxError};
use scanner::{Scanner, ScannerError};

use crate::{
    ast_enum::{AstNodeAccept, ExprAcceptMut, PrettyPrinter},
    enum_parser::Parser,
    interpreter::Interpreter,
};
// pub use reverse_polish_notation_visitor::Rpn;

/// A lox compiler and interpreter
#[derive(Debug)]
pub struct Lox {
    had_error: bool,
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            had_error: false,
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(&mut self, path: String) {
        let source = fs::read_to_string(path).unwrap();
        self.run(source);
        if self.had_error {
            process::exit(65)
        }
    }

    pub fn run_prompt(&mut self) {
        let mut line = String::new();
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            stdout.write_all(b"> ").unwrap();
            stdout.flush().unwrap();

            let _bytes_read = stdin.read_line(&mut line).unwrap();
            self.run(line.clone());
            self.had_error = false;
            // Remove contents from the buffer ther have been processed
            line.clear();
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let (tokens, errors) = scanner.scan_tokens();
        self.had_error = !errors.is_empty();

        for err in errors {
            match err {
                ScannerError::UnrecognizedSymbol(line, char) => {
                    self.error(line, format!("unreconized character: {char}"))
                }
                ScannerError::UnterminatedString(line) => self.error(
                    line,
                    format!("Unmatch string literal started. Expected closing '\"'"),
                ),
            }
        }

        let mut parser = Parser::new(tokens);
        let Ok(ast) = parser.parse() else {
            self.had_error = true;
            return;
        };

        // let pretty_print = PrettyPrinter {};
        // pretty_print.print(&ast);

        // Use persistent interpreter to maintain state accross parses
        if let Err(output) = self.interpreter.interpret(ast) {
            self.had_error = true;
            println!("{output:?}");
        }
    }

    pub fn error(&mut self, line: u64, msg: String) {
        self.report(line, "".to_string(), msg)
    }

    fn report(&mut self, line: u64, location: String, msg: String) {
        let mut stderr = io::stderr();
        stderr
            .write_all(format!("[line {line}] Error {location}: {msg}\n").as_bytes())
            .unwrap();
        stderr.flush().unwrap();

        // Maybe move to the error function?
        self.had_error = true;
    }
}
