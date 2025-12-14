#![cfg_attr(debug_assertions, allow(unused))]

use crate::compiler::{
    token::TokenType,
    lexer::Lexer,
};

mod cli;
mod compiler;
mod error;
mod location;

fn main() {
    let arg_list = cli::get_arg_list();
    let args = cli::parse_args(arg_list);

    let source = read_file(&args.input_file);
    compile(&source);
}

fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap_or_else(|err_msg| {
        error::program_error(&format!("Cannot open file \"{file_name}\": {err_msg}"));
    })
}

fn compile(source: &str) {
        let mut lexer = Lexer::new(source);
    if let Err(err) = lexer.init() {
        error::print_error(&err);
    }

    loop {
        match lexer.advance() {
            Ok(token) => {
                print!("GOT TOKEN");
                #[cfg(debug_assertions)]
                {
                    print!(": {token:#?}");
                }
                println!();
                if let TokenType::EOF = &token.token_type {
                    break;
                }
            }
            Err(err) => {
                error::print_error(&err);
            }
        }
    }
}
