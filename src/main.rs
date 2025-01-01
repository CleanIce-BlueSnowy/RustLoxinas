use std::{env, fs, io, path, process};
use std::io::{Read, Write};
use crate::scanner::TokenScanner;

mod tokens;
mod scanner;
mod expr;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Loxinas 1.0.0 alpha [Developing]");
    if args.len() == 1 {
        if let Err(err) = run_interact() {
            eprintln!("{err}");
            process::exit(64);
        }
    } else if args.len() == 2 {
        if let Err(err) = run_file(&args[1]) {
            eprintln!("{err}");
            process::exit(68);
        }
    } else {
        println!("Usage: RustLoxinas");
    }
}

fn run_interact() -> Result<(), String> {
    println!("Interactive Mode [Type EOF (Ctrl + Z or Ctrl + D) to exit]");
    let input = io::stdin();
    let mut line = String::new();
    loop {
        line.clear();
        print!("> ");
        if let Err(err) = io::stdout().flush() {
            throw_error(format!("STDOUT Flushing Error!\n  Error Message: {err}"))?
        }
        match input.read_line(&mut line) {
            Ok(read) if read == 0 => break,
            Err(err) => throw_error(format!("STDIN Reading Error!\n  Error Message: {err}"))?,
            _ => (),
        }
        let mut scanner = TokenScanner::new(line.trim().to_string());
        if let Err(err) = scanner.scan_tokens() {
            println!("{err}");
            continue;
        }
        let (tokens, _source) = scanner.get_tokens_and_source();
        for token in tokens {
            println!("{token:?}");
        }
    }
    return Ok(());
}

fn run_file(file_name: &str) -> Result<(), String> {
    if !path::Path::new(file_name).exists() {
        eprintln!("File '{file_name}' does not exist.");
        process::exit(65);
    }
    let mut file;
    match fs::File::open(file_name) {
        Ok(res) => file = res,
        Err(err) => {
            eprintln!("Cannot open file '{file_name}': {err}");
            process::exit(66);
        }
    }
    let mut code = String::new();
    if let Err(err) = file.read_to_string(&mut code) {
        eprintln!("FILE READING ERROR: {err}");
        process::exit(67);
    }
    let mut scanner = TokenScanner::new(code);
    scanner.scan_tokens()?;
    let (tokens, _source) = scanner.get_tokens_and_source();
    for token in tokens {
        println!("{token:?}");
    }
    return Ok(());
}

fn throw_error(msg: String) -> Result<(), String> {
    Err(format!("Program Error: {msg}"))
}
