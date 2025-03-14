//! RustLoxinas 前端编译

use std::{env, process};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[cfg(debug_assertions)]
use RustLoxinas::ast_printer::AstPrinter;

use RustLoxinas::errors::{ErrorList, print_all_errors};
use RustLoxinas::front_compiler::FrontCompiler;
use RustLoxinas::parser::Parser;
use RustLoxinas::scanner::TokenScanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Loxinas 1.0.0 alpha [Developing] {{Compiler}}");

    if args.len() < 2 {
        println!("Usage: loxinas compile (or loxc) <source file> [other args]");
        println!("[help: Type 'loxinas help compile' or 'loxc --help' to get more help information]");
        return;
    }

    if "--help" == &args[1] {
        unimplemented!("Help information is not implemented!");
    }

    let source_path: &str = &args[1];
    let mut output_path: Option<&str> = None;
    let mut i = 2usize;
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
        &(Path::new(path).file_stem().and_then(|name| name.to_str()).unwrap_or("main").to_string() + ".loxc")
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
