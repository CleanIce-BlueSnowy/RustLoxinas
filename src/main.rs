#![cfg_attr(debug_assertions, allow(unused))]

mod cli;
mod compiler;
mod error;
mod location;

use crate::compiler::{
    lexer::Lexer,
    token::TokenType,
    parser::Parser,
};
use crate::error::ParseError;

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
        error::print_error(&ParseError::from(err));
        return;
    }
    let mut parser = Parser::new(&mut lexer);

    match parser.expression() {
        Ok(expr) => {
            println!("GOT EXPRESSION");
            #[cfg(debug_assertions)]
            {
                println!("{expr:#?}");
            }
        }
        Err(err) => error::print_error(&err),
    }
}
