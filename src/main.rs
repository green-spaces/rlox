use std::{env, process, fs, io::{self, Write}};

fn main() {
    let mut args = env::args();
    println!("args: {args:?}");
    println!("args len: {}", args.len());

    match args.len() {
        0 | 1 => run_prompt(),
        2 => run_file(args.nth(1).unwrap()),
        _ => {
            println!("Usage: rlox [script]");
            process::exit(64);    
        }
    }
}

fn run_file(path: String) {
    let source = fs::read_to_string(path).unwrap();
    run(source)
}

fn run_prompt() {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        stdout.write_all(b"> ").unwrap();
        stdout.flush().unwrap();

        let _bytes_read = stdin.read_line(&mut buf).unwrap();
        run(buf.clone())
    }
}

fn run(source: String) {
    println!("{source:?}");
}