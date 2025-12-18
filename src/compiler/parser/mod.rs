mod expr;

use crate::compiler::lexer::Lexer;
use crate::error::{LexicalError, LoxinasError};
use crate::location::Location;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        Self {
            lexer
        }
    }
}

pub struct ParseError {
    location: Location,
    msg: String,
}

impl LoxinasError for ParseError {
    fn get_type() -> &'static str {
        "Parse Error"
    }

    fn get_location(&self) -> &Location {
        &self.location
    }

    fn get_msg(&self) -> &str {
        &self.msg
    }

    fn take_info(self) -> (Location, String) {
        (self.location, self.msg)
    }
}

impl From<LexicalError> for ParseError {
    fn from(value: LexicalError) -> Self {
        let data = value.take_info();
        Self {
            location: data.0,
            msg: data.1,
        }
    }
}
