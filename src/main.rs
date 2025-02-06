//! Loxinas 编译器、反汇编器、虚拟机的 Rust 语言实现

#![cfg_attr(debug_assertions, allow(dead_code))]

#[cfg(debug_assertions)]
use crate::ast_printer::AstPrinter;

use std::{env, process};
use std::fs::File;
use std::io::{Read, Write};
use crate::compiler::Compiler;
use crate::disassembler::disassemble_file;
use crate::parser::Parser;
use crate::position::Position;
use crate::resolver::Resolver;
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
    let (_const_pool, chunk) = compile_code(source)?;
    
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
fn compile_code(source: String) -> Result<(Vec<u8>, Vec<u8>), String> {
    let lines: Vec<&str> = source.split('\n').collect();
    
    let mut scanner = TokenScanner::new(&source);  // 词法分析
    if let Err(err) = scanner.scan_tokens() {
        return Err(print_error("Lexical Error", &lines, &err.message, &err.pos))
    }
    let tokens = scanner.get_tokens();
    
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
    
    #[cfg(debug_assertions)]
    {
        let mut printer = AstPrinter::new();
        println!("== AST ==");  // 开发中，先打印语法树
        println!("{}", printer.print(&expr));
    }
    
    let mut resolver = Resolver::new();  // 语义分析
    
    #[cfg(debug_assertions)]
    let res;
    
    match resolver.resolve_expr(&expr) {
        Ok(_temp) => {
            #[cfg(debug_assertions)]
            { res = _temp; }
            ()
        }
        Err(err) => return Err(print_error("Compile Error", &lines, &err.message, &err.pos)),  // 返回编译错误
    }
    
    #[cfg(debug_assertions)]
    {
        println!("== Expr Result ==");  // 开发中，先打印分析结果
        println!("{:?}", res.res_type);
    }
    
    let mut compiler = Compiler::new(resolver.expr_ope_type, resolver.expr_res_type);
    compiler.compile_expression(&expr);
    
    return Ok((compiler.const_pool, compiler.chunk));
}

/// 执行文件
fn run_file(path: &str) -> Result<(), String> {
    let mut file;
    match File::open(path) {
        Ok(temp) => file = temp,
        Err(err) => return Err(format!("Cannot open file '{}'! Error message: {}", path, err)),
    }
    let mut buffer: Vec<u8> = Vec::new();
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

/** 打印错误（返回字符串）

接受错误类型、代码行、错误位置

错误格式（单行）：

```
<Error Type>: line ? at ?-?: <Error Message>
  |> This is the code and here leads an error
                          ^^^^
```

错误格式（两行）：
```
<Error Type>: from (line ? at ?) to (line ? at ?): <Error Message>
  |> This is the first line and here begins the error
                                ^^^^^^^^^^^^^^^^^^^^^
  |> This is the last line and here ends the error
     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

错误格式（多行）：
```
<Error Type>: from (line ? at ?) to (line ? at ?): <Error Message>
  |> This is the first line and here begins the error
                                ^^^^^^^^^^^^^^^^^^^^^
  |> ...
  |> This is the last line and here ends the error
     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```
 */
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

/** 打印运行时错误

功能不完全，因为字节码符号表尚未完成

错误格式：

```
Runtime Error: <Error Message>
```
 */
fn print_runtime_error(msg: &str) -> String {
    format!("Runtime Error: {}", msg)
}
