mod lox;

use lox::tokens;
use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    process::exit,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    if args_len > 2 {
        println!("Usage: lox [filepath]");
        exit(64);
    } else if args_len == 2 {
        run_file(&args[1]);
    } else if args_len == 1 {
        run_prompt();
    }
}

fn run_file(filepath: &str) {
    let mut file = File::open(filepath).unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf);
    run(buf);
}

fn run_prompt() {
    let mut buf = vec![];
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let bytes = io::stdin().read_to_end(&mut buf).unwrap(); // brittle
        if bytes == 0 {
            break;
        }
        run(buf.clone()); // REPL code is small
        buf.clear();
    }
}

fn run(source: Vec<u8>) {}

fn report(line: i32, location: String, message: String) {
    eprintln!("[line {line}] Error {location}: {message}");
}

pub fn error(line: i32, message: String) {
    report(line, "".into(), message);
}
