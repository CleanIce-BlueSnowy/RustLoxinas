// RustLoxinas 反汇编

use std::{env, process};
use RustLoxinas::disassembler::disassemble_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Loxinas 1.0.0 alpha [Developing] {{Disassembler}}");
    
    if args.len() < 2 {
        println!("Usage: loxinas disassemble (or loxdasm) <source file> [other args]");
        println!("[help: Type 'loxinas help disassemble' or 'loxdasm --help' to get more help information]");
        return;
    }

    if "--help" == &args[1] {
        unimplemented!("Help information is not implemented!");
    }
    
    let source_path: &str = &args[1];
    // 启动反汇编
    println!("Disassembling...");
    if let Err(err) = disassemble_file(source_path) {
        eprintln!("{}", err);
        process::exit(64);
    }
    println!("Disassemble Finished!");
}
