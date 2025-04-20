// RustLoxinas 反汇编

#![allow(static_mut_refs)]

use std::{env, io, process};
use std::fs::File;
use std::path::Path;

use RustLoxinas::disassembler::disassemble_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Loxinas 1.0.0 alpha [Developing] {{Disassembler}}");
    
    #[cfg(debug_assertions)]
    {
        println!("--- Debug Mode ---");
    }
    
    if args.len() < 2 {
        println!("Usage: loxinas disassemble (or loxdasm) <source file> [arguments]");
        println!("[help: Type 'loxinas help disassemble' or 'loxdasm --help' to get more help information]");
        return;
    }

    if "--help" == &args[1] {
        println!("HELP: The disassembler of Loxinas. It can disassemble the Loxinas byte-code, show them in a human-readable way.");
        println!("Usage: loxinas disassemble (or loxdasm) <source file> [arguments]");
        println!("---------------");
        println!("<source file>: Loxinas byte-code file (.loxc file)");
        println!("Arguments:");
        println!("    --output | -o <output file>: Write the human-readable byte-code into a <output file> (.loxa file by default).");
        println!("        If you don't set this argument, the result will be printed to the console.");
        return;
    }
    
    let source_path: &str = &args[1];
    let mut i = 2usize;
    let mut file = None;

    while i < args.len() {
        let arg: &str = &args[i];
        match arg {
            "-o" | "--output" => {  // 输出文件
                if i + 1 >= args.len() {
                    eprintln!("Expect output file path after '-o'.");
                    process::exit(2);
                }
                i += 1;
                let file_path = Path::new(&args[i]);
                file = Some(
                    match File::create(
                        if let None = file_path.extension() {
                            format!("{}.loxa", &args[i])
                        } else {
                            args[i].clone()
                        }
                    ) {
                        Ok(temp) => temp,
                        Err(err) => {
                            eprintln!("Cannot open output file '{}'! Error message: {}", args[i], err);
                            process::exit(1);
                        }
                    });
            }
            _ => {
                eprintln!("Unknown arg: '{}'", arg);
                process::exit(1);
            }
        }
        i += 1;
    }

    // 启动反汇编
    println!("Disassembling...");

    let mut stdout = io::stdout();
    if let Err(err) = disassemble_file(source_path, if let Some(file) = &mut file { file } else { &mut stdout }) {
        eprintln!("{}", err);
        process::exit(64);
    }

    println!("Disassemble Finished!");
}
