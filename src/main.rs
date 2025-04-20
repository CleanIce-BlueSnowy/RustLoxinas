//! Loxinas 编译器、反汇编器、虚拟机的 Rust 语言实现

#![cfg_attr(debug_assertions, allow(dead_code))]
#![allow(non_snake_case)]

use std::{env, process};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Loxinas 1.0.0 alpha [Developing]");
        
        #[cfg(debug_assertions)]
        {
            println!("--- Debug Mode ---");
        }
        
        println!("Usage: loxinas <operation> [other args]");
        println!("[help: Type 'loxinas help' to get more help information]");
        return;
    }
    
    let operation: &str = &args[1];
    let mut command = Command::new(
        match operation {
            "help" => "./loxhelp",
            "compile" => "./loxc",
            "disassemble" => "./loxdasm",
            "run" => "./loxr",
            _ => {
                eprintln!("Unknown operation: '{}'", operation);
                process::exit(1);
            }
        }
    );
    
    if args.len() >= 3 {
        command.args(&args[2..]);
    }
    
    let mut child = command.spawn().expect("Failed to spawn the process.");
    let status = child.wait().expect("Failed to wait the process.");
    if status.success() {
        return;
    } else {
        process::exit(status.code().unwrap());
    }
}
