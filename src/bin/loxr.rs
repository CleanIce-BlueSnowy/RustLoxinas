// RustLoxinas 虚拟机运行

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{env, process};
use RustLoxinas::errors::print_runtime_error;
use RustLoxinas::vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();

    #[cfg(debug_assertions)]
    {
        println!("Loxinas 1.0.0 alpha [Developing] {{Runner}}");
        println!("--- Debug Mode ---");
    }

    if args.len() < 2 {
        println!("Usage: loxinas run (or loxr) <source file> [other args]");
        println!("[help: Type 'loxinas help run' or 'loxr --help' to get more help information]");
        return;
    }

    if "--help" == &args[1] {
        println!("Usage: loxinas run (or loxr) <source file> [other args]");
        println!("---------------");
        println!("<source file>: Loxinas byte-code file (.loxc file)");
        println!("    If you only type the stem name, the extension name is '.loxc' by default.");
        println!("Arguments:");
        println!("    (None Yet)");
        return;
    }

    let file_path: &str = {
        let path = Path::new(&args[1]);
        if let None = path.extension() {
            &format!("{}.loxc", args[1])
        } else {
            &args[1]
        }
    };

    // 启动虚拟机运行
    if let Err(err) = run_file(file_path) {
        eprintln!("{}", err);
        process::exit(70);
    }
}

/// 执行文件
fn run_file(path: &str) -> Result<(), String> {
    let mut file;
    match File::open(path) {
        Ok(temp) => file = temp,
        Err(err) => {
            return Err(format!(
                "Cannot open file '{}'! Error message: {}",
                path, err
            ))
        }
    }
    let mut buffer: Vec<u8> = vec![];
    if let Err(err) = file.read_to_end(&mut buffer) {
        return Err(format!(
            "Cannot read file '{}'! Error message: {}",
            path, err
        ));
    }

    // 运行代码
    run_code(&buffer)?;

    Ok(())
}

/// 执行字节码
fn run_code(code: &[u8]) -> Result<(), String> {
    // 创建虚拟机
    let mut vm = VM::new(code);

    // 执行
    if let Err(err) = vm.run() {
        return Err(print_runtime_error(&err.message));
    }

    Ok(())
}
