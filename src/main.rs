#![cfg_attr(debug_assertions, allow(dead_code))]

use std::{env, fs, io, path, process};
use std::io::{Read, Write};
use crate::ast_printer::AstPrinter;
use crate::parser::Parser;
use crate::position::Position;
use crate::resolver::Resolver;
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
mod object;
mod resolver;
mod resolver_expr;
mod ast_printer_expr;
mod instr;
mod position;

fn main() {
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
        if line.is_empty() {
            continue;
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
    let lines: Vec<&str> = source.split('\n').collect();
    
    let mut scanner = TokenScanner::new(&source);  // 词法分析
    if let Err(err) = scanner.scan_tokens() {
        return Err(print_error("Lexical Error", &lines, &err.message, &err.pos))
    }
    let tokens = scanner.get_tokens_and_source();
    
    #[cfg(debug_assertions)]
    {
        println!("== All Tokens ==");  // 开发中，先打印所有令牌
        for token in &tokens {
            println!("{token:?}");
        }
    }
    
    let mut parser = Parser::new(tokens);  // 语法分析
    let expr;
    match parser.parse() {
        Ok(temp) => expr = temp,
        Err(err) => return Err(print_error("Syntax Error", &lines, &err.message, &err.pos)),  // 返回语法错误
    }
    let mut printer = AstPrinter::new();
    
    #[cfg(debug_assertions)]
    {
        println!("== AST ==");  // 开发中，先打印语法树
        println!("{}", printer.print(&expr));
    }
    
    let mut resolver = Resolver::new();  // 语义分析
    let res;
    match resolver.resolve_expr(&expr) {
        Ok(temp) => res = temp,
        Err(err) => return Err(print_error("Compile Error", &lines, &err.message, &err.pos)),  // 返回编译错误
    }
    
    #[cfg(debug_assertions)]
    {
        println!("== Expr Result ==");  // 开发中，先打印分析结果
        println!("{:?}", res.expr_type);
    }
    
    return Ok(());
}

/// 抛出程序错误
fn throw_error(msg: String) -> Result<(), String> {
    Err(format!("Program Error: {msg}"))
}

/// 打印错误（返回字符串）
/// 
/// 接受错误类型、代码行、错误位置
/// 
/// 错误格式（单行）：
/// 
/// ```
/// <Error Type>: line ? at ?-?: <Error Message>
///   |> This is the code and here leads an error
///                           ^^^^
/// ```
/// 
/// 错误格式（两行）：
/// ```
/// <Error Type>: from (line ? at ?) to (line ? at ?): <Error Message>
///   |> This is the first line and here begins the error
///                                 ^^^^^^^^^^^^^^^^^^^^^
///   |> This is the last line and here ends the error
///      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
/// 
/// 错误格式（多行）：
/// ```
/// <Error Type>: from (line ? at ?) to (line ? at ?): <Error Message>
///   |> This is the first line and here begins the error
///                                 ^^^^^^^^^^^^^^^^^^^^^
///   |> ...
///   |> This is the last line and here ends the error
///      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
fn print_error(error_type: &str, lines: &Vec<&str>, message: &str, pos: &Position) -> String {
    let mut res = if pos.start_line == pos.end_line {  // 根据是否在同一行给出不同的输出格式
        format!("{}: line {} at {}-{}: {}\n", error_type, pos.start_line, pos.start_idx + 1, pos.end_idx, message)
    } else {
        format!("{}: from (line {} at {}) to (line {} at {}): {}\n", error_type, pos.start_line, pos.start_idx + 1, pos.end_line, pos.end_idx + 1, message)
    };
    
    let line = lines[pos.start_line - 1];  // 起始行
    res.push_str(&format!("  |> {}\n     ", line));
    let end_idx = if pos.start_line == pos.end_line {  // 确认起始行位置提示终止位置
        pos.end_idx
    } else {
        let chars: Vec<char> = line.chars().collect();
        chars.len() - 1
    };
    
    // 打印起始行位置提示
    for _i in 0..pos.start_idx {
        res.push(' ');
    }
    for _i in pos.start_idx..end_idx {
        res.push('^');
    }
    res.push('\n');
    
    // 若错误不在一行以内
    if pos.start_line != pos.end_line {
        if pos.end_line - pos.start_line > 1 {  // 错误行数大于 2 行，则省略中间行
            res.push_str("  |> ...\n");
        }
        let line = lines[pos.end_line - 1];  // 终止行
        res.push_str(&format!("  |> {}\n     ", line));
        for _i in 0..pos.end_idx {  // 打印终止行位置提示
            res.push('^');
        }
        res.push('\n');
    }
    
    return res;
}
