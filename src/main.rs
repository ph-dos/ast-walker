use logos::Logos;

use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    process::exit,
};

#[derive(Logos, Debug, PartialEq)]
enum Token {
    // #[token(";")]
    // Semicolon,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}

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
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    run(&buf);
}

fn run_prompt() {
    let mut buf = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let bytes = io::stdin().read_line(&mut buf).unwrap();
        if bytes == 0 {
            break;
        }
        run(&buf);
        buf.clear();
    }
}

fn run(source: &str) {
    let mut lexer = Token::lexer(source);
    while let Some(_token) = lexer.next() {
        println!("{}", lexer.slice());
    }
}
