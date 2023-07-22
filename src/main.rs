use rlox::Lox;
use std::{env, process};

fn main() {
    let mut args = env::args();

    let mut lox = Lox::new();

    match args.len() {
        0 | 1 => lox.run_prompt(),
        2 => lox.run_file(args.nth(1).unwrap()),
        _ => {
            println!("Usage: rlox [script]");
            process::exit(64);
        }
    }
}
