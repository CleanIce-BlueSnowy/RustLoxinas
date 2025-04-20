//! RustLoxinas 帮助

use std::process::Command;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: loxinas help (or loxhelp) <operation> [other args]");
        return;
    }

    let operation: &str = &args[1];
    let mut command = Command::new(match operation {
        "compile" => "./loxc",
        "disassemble" => "./loxdasm",
        "run" => "./loxr",
        _ => {
            eprintln!("Unknown operation: '{}'", operation);
            process::exit(1);
        }
    });
    command.arg("--help");

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
