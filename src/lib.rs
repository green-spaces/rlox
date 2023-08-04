// pub mod ast;
// pub mod ast_struct_macros;
// pub mod ast_visitor;
pub mod scanner;
pub mod token;
// pub mod parser;
pub mod ast_enum;
pub mod enum_parser;
pub mod error;
pub mod reverse_polish_notation_visitor;

use error::{RunTimeError, SyntaxError};

use scanner::{Scanner, ScannerError};

use std::{
    fs,
    io::{self, Write},
    process,
};

use crate::{
    ast_enum::{AstNodeAccept, PrettyPrinter},
    enum_parser::Parser,
    reverse_polish_notation_visitor::Rpn,
};

/// A lox compiler and interpreter
#[derive(Debug)]
pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
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
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let (tokens, errors) = scanner.scan_tokens();
        self.had_error = !errors.is_empty();

        for token in tokens.iter() {
            println!("{token:?}");
        }

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
        let Ok(ast) = parser.parse() else { return; };

        let pretty_print = PrettyPrinter {};
        let ast_str = ast.accept(pretty_print);
        println!("{ast_str}");

        //        let rpn = Rpn {};
        //        let rpn_str = ast.accept(rpn);
        //        println!("{rpn_str}");
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
