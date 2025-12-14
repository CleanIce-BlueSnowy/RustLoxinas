use crate::location::Location;

pub use crate::compiler::lexer::LexicalError;

pub trait LoxinasError {
    fn get_type() -> &'static str;
    fn get_location(&self) -> &Location;
    fn get_msg(&self) -> &str;
}

pub fn print_error<ErrorType: LoxinasError>(err: &ErrorType) {
    print_error_msg(ErrorType::get_type(), Some(&err.get_location().to_string()), err.get_msg());
}

pub fn program_error(msg: &str) -> ! {
    print_error_msg("Program Error", None, msg);
    exit(1);
}

pub fn exit(code: i32) -> ! {
    std::process::exit(code);
}

fn print_error_msg(err_type: &str, before_msg: Option<&str>, msg: &str) {
    if let Some(before_msg) = before_msg {
        eprintln!("{err_type} {before_msg}: {msg}");
    } else {
        eprintln!("{err_type}: {msg}");
    }
}
