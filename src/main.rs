//! Loxinas 编译器、反汇编器、虚拟机的 Rust 语言实现

#![cfg_attr(debug_assertions, allow(dead_code))]

use std::{env, process};
use std::fs::File;
use std::io::{Read, Write};

#[cfg(debug_assertions)]
use crate::ast_printer::AstPrinter;
use crate::disassembler::disassemble_file;
use crate::errors::{ErrorList, print_all_errors, print_runtime_error};
use crate::front_compiler::FrontCompiler;
use crate::parser::Parser;
use crate::scanner::TokenScanner;
use crate::vm::VM;

#[cfg(debug_assertions)]
mod ast_printer;

mod tokens;
mod scanner;
mod expr;
mod data;
mod parser;
mod types;
mod object;
mod resolver;
mod instr;
mod position;
mod compiler;
mod disassembler;
mod byte_handler;
mod vm;
mod assistance;
mod stmt;
mod errors;
mod front_compiler;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Loxinas 1.0.0 alpha [Developing]");
        println!("Usage: loxinas <operation> [other args]");
        println!("[help: Type 'loxinas help' to get more help information]");
        return;
    }
    let operation: &str = &args[1];
    match operation {
        "help" => unimplemented!("Help information is not implemented!"),
        "compile" => {  // 编译
            println!("Loxinas 1.0.0 alpha [Developing]");
            if args.len() < 3 {
                println!("Usage: loxinas compile <source file> [other args]");
                println!("[help: Type 'loxinas help compile' to get more help information]");
                return;
            }
            let source_path: &str = &args[2];
            let mut output_path: Option<&str> = None;
            let mut i = 3usize;
            while i < args.len() {
                let arg: &str = &args[i];
                match arg {
                    "-o" | "--output" => {  // 输出文件
                        if i + 1 >= args.len() {
                            eprintln!("Expect output file path after '-o'.");
                            process::exit(2);
                        }
                        i += 1;
                        output_path = Some(&args[i]);
                    }
                    _ => {
                        eprintln!("Unknown arg: '{}'", arg);
                        process::exit(1);
                    }
                }
                i += 1;
            }
            // 启动编译
            println!("Compiling...");
            if let Err(err) = compile_file(source_path, output_path) {
                eprintln!("{}", err);
                process::exit(64);
            }
            println!("Compile Finished!");
        }
        "disassemble" => {  // 反汇编
            println!("Loxinas 1.0.0 alpha [Developing]");
            if args.len() < 3 {
                println!("Usage: loxinas disassemble <source file> [other args]");
                println!("[help: Type 'loxinas help disassemble' to get more help information]");
                return;
            }
            let source_path: &str = &args[2];
            // 启动反汇编
            println!("Disassembling...");
            if let Err(err) = disassemble_file(source_path) {
                eprintln!("{}", err);
                process::exit(64);
            }
            println!("Disassemble Finished!");
        }
        "run" => {  // 执行
            if args.len() < 3 {
                println!("Usage: loxinas run <byte code file> [other args]");
                println!("[help: Type 'loxinas help run' to get more help information]");
                return;
            }
            let file_path: &str = &args[2];
            // 启动虚拟机运行
            if let Err(err) = run_file(file_path) {
                eprintln!("{}", err);
                process::exit(70);
            }
        }
        _ => {
            eprintln!("Unknown operation: '{}'", operation);
            process::exit(1);
        }
    }
}

/// 编译文件
fn compile_file(path: &str, output_path: Option<&str>) -> Result<(), String> {
    // 读取源文件
    let mut file;
    match File::open(path) {
        Ok(temp) => file = temp,
        Err(err) => return Err(format!("Cannot open file '{}'! Error message: {}", path, err)),
    }
    let mut source = String::new();
    if let Err(err) = file.read_to_string(&mut source) {
        return Err(format!("Cannot read file '{}'! Error message: {}", path, err));
    }
    
    // 编译
    let chunk = compile_code(source)?;
    
    // 写入目标文件
    let output_file_path = if let Some(output_path) = output_path {  // 用户提供了输出文件
        output_path
    } else {  // 默认输出文件
        &(path.to_string() + ".loxc")
    };
    let mut file;
    match File::create(output_file_path) {
        Ok(temp) => file = temp,
        Err(err) => return Err(format!("Cannot open output file '{}'! Error message: {}", output_file_path, err)),
    }
    if let Err(err) = file.write_all(&chunk) {  // 写入字节码
        return Err(format!("Cannot write output file '{}'! Error message: {}", output_file_path, err));
    }
    
    return Ok(());
}

/// 编译代码
fn compile_code(source: String) -> Result<Vec<u8>, String> {
    let lines: Vec<&str> = source.lines().collect();
    
    let scanner = TokenScanner::new(&source);  // 词法分析
    let tokens = match scanner.scan_tokens() {
        Ok(temp) => temp,
        Err(errs) => return Err(print_all_errors(&lines, ErrorList::LexicalErrors(&errs))),
    };
    
    #[cfg(debug_assertions)]
    {
        println!("== All Tokens ==");  // 开发中，先打印所有令牌
        for token in &tokens {
            println!("{token:?}");
        }
    }
    
    let mut parser = Parser::new(tokens);  // 语法分析
    let statements = match parser.parse() {
        Ok(temp) => temp,
        Err(errs) => return Err(print_all_errors(&lines, ErrorList::SyntaxErrors(&errs))),
    };
    
    #[cfg(debug_assertions)]
    {
        let mut printer = AstPrinter::new();
        println!("== AST ==");  // 开发中，先打印语法树
        println!("{}", printer.print(&statements));
    }
    
    // 前端编译
    let mut front_compiler = FrontCompiler::new(&statements);
    
    return match front_compiler.compile() {
        Ok(codes) => Ok(codes),
        Err(errs) => Err(print_all_errors(&lines, ErrorList::CompileErrors(&errs))),
    };
}

/// 执行文件
fn run_file(path: &str) -> Result<(), String> {
    let mut file;
    match File::open(path) {
        Ok(temp) => file = temp,
        Err(err) => return Err(format!("Cannot open file '{}'! Error message: {}", path, err)),
    }
    let mut buffer: Vec<u8> = vec![];
    if let Err(err) = file.read_to_end(&mut buffer) {
        return Err(format!("Cannot read file '{}'! Error message: {}", path, err));
    }
    
    // 运行代码
    run_code(&buffer)?;
    
    return Ok(());
}

/// 执行字节码
fn run_code(code: &[u8]) -> Result<(), String> {
    // 创建虚拟机
    let mut vm = VM::new(code);
    
    // 执行
    if let Err(err) = vm.run() {
        return Err(print_runtime_error(&err.message));
    }
    
    return Ok(());
}
