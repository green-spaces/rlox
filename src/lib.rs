pub mod ast;
pub mod ast_struct_macros;
pub mod ast_visitor;
pub mod scanner;
pub mod token;

use scanner::Scanner;

use std::{
    fs,
    io::{self, Write},
    process,
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

        for token in tokens {
            println!("{token:?}");
        }

        println!("Found {} errors", errors.len());
        println!("{errors:#?}");
    }

    pub fn error(&mut self, line: u64, msg: String) {
        self.report(line, "".to_string(), msg)
    }

    fn report(&mut self, line: u64, location: String, msg: String) {
        let mut stderr = io::stderr();
        stderr
            .write_all(format!("[line {line}] Error {location}: {msg}").as_bytes())
            .unwrap();
        stderr.flush().unwrap();

        // Maybe move to the error function?
        self.had_error = true;
    }
}
