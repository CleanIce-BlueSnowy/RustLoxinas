use std::{env, fs, io, path, process};
use std::io::{Read, Write};
use crate::scanner::TokenScanner;

mod tokens;
mod scanner;
mod expr;
mod data;

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
        println!("Usage: loxinas [file]");
    }
}

/// 交互模式
fn run_interact() -> Result<(), String> {
    println!("Interactive Mode [Type EOF (Ctrl + Z or Ctrl + D) to exit]");
    let input = io::stdin();
    let mut line = String::new();
    loop {
        line.clear();
        print!("> ");
        if let Err(err) = io::stdout().flush() {  // 立即刷新
            throw_error(format!("STDOUT Flushing Error!\n  Error Message: {err}"))?
        }
        match input.read_line(&mut line) {
            Ok(read) if read == 0 => break,  // 控制台 EOF
            Err(err) => throw_error(format!("STDIN Reading Error!\n  Error Message: {err}"))?,
            _ => {}
        }
        let mut scanner = TokenScanner::new(line.trim().to_string());  // 词法解析
        if let Err(err) = scanner.scan_tokens() {  // 交互模式的错误不应该继续回溯，而是输出后继续交互
            println!("{err}");
            continue;
        }
        let (tokens, _source) = scanner.get_tokens_and_source();
        for token in tokens {  // 开发中，先打印所有令牌
            println!("{token:?}");
        }
    }
    return Ok(());
}

/// 运行文件
fn run_file(file_name: &str) -> Result<(), String> {
    if !path::Path::new(file_name).exists() {
        throw_error(format!("File '{file_name}' does not exist."))?;
    }
    let mut file;
    match fs::File::open(file_name) {
        Ok(res) => file = res,
        Err(err) => {
            throw_error(format!("Cannot open file '{file_name}': {err}"))?;
            panic!("Function throw_error() didn't throw an error.");  // 为了避免编译器报错 `file` 可能未初始化
        }
    }
    let mut code = String::new();
    if let Err(err) = file.read_to_string(&mut code) {  // 一次性读取所有源代码
        throw_error(format!("FILE READING ERROR: {err}"))?;
    }
    let mut scanner = TokenScanner::new(code);  // 词法分析
    scanner.scan_tokens()?;
    let (tokens, _source) = scanner.get_tokens_and_source();
    for token in tokens {  // 开发中，先打印所有令牌
        println!("{token:?}");
    }
    return Ok(());
}

/// 抛出程序错误
fn throw_error(msg: String) -> Result<(), String> {
    Err(format!("Program Error: {msg}"))
}
