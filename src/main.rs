#![cfg_attr(debug_assertions, allow(dead_code))]

use std::{env, fs, io, path, process};
use std::io::{Read, Write};
use crate::ast_printer::AstPrinter;
use crate::parser::Parser;
use crate::scanner::TokenScanner;

mod tokens;
mod scanner;
mod expr;
mod data;
mod ast_printer;
mod parser;
mod parser_assistance;
mod parser_expr;
mod types;

fn main() {
    #[cfg(debug_assertions)]
    {
        ast_printer::test();
    }
    let args: Vec<String> = env::args().collect();
    println!("Loxinas 1.0.0 alpha [Developing]");
    if args.len() == 1 {  // 无控制台参数
        if let Err(err) = run_interact() {
            eprintln!("{err}");
            process::exit(64);
        }
    } else if args.len() == 2 {  // 运行文件
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
        if let Err(err) = run_code(line.trim().to_string()) { // 交互模式的错误不应该继续回溯，而是输出后继续交互
            println!("{err}");
            continue;
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
    run_code(code)?;
    return Ok(());
}

fn run_code(source: String) -> Result<(), String> {
    let mut scanner = TokenScanner::new(source);  // 词法分析
    scanner.scan_tokens()?;
    let (tokens, _source) = scanner.get_tokens_and_source();
    for token in &tokens {  // 开发中，先打印所有令牌
        println!("{token:?}");
    }
    let lines: Vec<&str> = _source.split('\n').collect();
    let mut parser = Parser::new(tokens);  // 语法分析
    match parser.parse() {
        Ok(expr) => {
            let mut printer = AstPrinter::new();
            println!("{}", printer.print(&expr));  // 开发中，先打印语法树
        }
        Err(err) => print_error("Syntax Error", &lines, &err.message, err.token.line, err.token.start, err.token.end),  // 打印语法错误'a
    }
    return Ok(());
}

/// 抛出程序错误
fn throw_error(msg: String) -> Result<(), String> {
    Err(format!("Program Error: {msg}"))
}

/// 打印错误
fn print_error(error_type: &str, lines: &Vec<&str>, message: &str, line: usize, start: usize, end: usize) {
    println!("{}: line {} at {}-{}: {}", error_type, line, start, end, message);
    println!("  |> {}", lines[line - 1]);
    print!("  |> ");
    for _i in 0..start {
        print!(" ");
    }
    for _i in start..end {
        print!("^");
    }
    println!();
}
