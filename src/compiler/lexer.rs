use crate::error::LoxinasError;
use crate::location::Location;

use super::token::{Token, TokenType};

pub struct Lexer<'a> {
    source: &'a str,
    chars: Vec<char>,
    prev: Token,
    current: Token,
    pos: (usize, usize),  // (start, end)
    location: Location,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().collect(),
            prev: Token::make_eof(),
            current: Token::make_eof(),
            pos: (0, 0),
            location: Location::create(1, 0, 1, 0),
        }
    }

    pub fn init(&mut self) -> Result<(), LexicalError> {
        self.scan_token()
    }

    pub fn advance(&mut self) -> Result<&Token, LexicalError> {
        self.scan_token()?;
        Ok(&self.prev)
    }

    pub fn previous(&self) -> &Token {
        &self.prev
    }

    pub fn peek(&self) -> &Token {
        &self.current
    }

    fn scan_token(&mut self) -> Result<(), LexicalError> {
        self.skip_whitespace();
        self.fresh_pos();

        let token = match self.advance_char() {
            '\0' => self.make_token(TokenType::EOF),
            '+' => self.make_token(TokenType::OpeAdd),
            '-' => self.make_token(TokenType::OpeSub),
            '*' => self.make_token(TokenType::OpeStar),
            '/' => self.make_token(TokenType::OpeSlash),
            '=' => self.make_token(TokenType::OpeEqual),
            ch if ch.is_alphabetic() || ch == '_' => {
                while self.peek_char().is_alphanumeric() || self.peek_char() == '_' {
                    self.advance_char();
                }
                self.make_token(TokenType::Identifier(self.copy_string()))
            }
            ch => return self.error(format!("Unknown character: `{ch}`")),
        };

        self.prev = std::mem::replace(&mut self.current, token);

        Ok(())
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek_char() {
                '\n' => {
                    self.new_line();
                    self.advance_char();
                }
                ch if ch.is_whitespace() => {
                    self.advance_char();
                }
                _ => break,
            }
        }
    }

    fn copy_string(&self) -> String {
        String::from_iter(&self.chars[self.pos.0..self.pos.1])
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            location: self.location.clone(),
        }
    }

    fn fresh_pos(&mut self) {
        self.pos.0 = self.pos.1;
        self.location.start = self.location.end.clone();
    }

    fn new_line(&mut self) {
        self.location.end.line += 1;
        self.location.end.col = 0;
    }

    fn source_at_end(&self) -> bool {
        self.pos.1 >= self.chars.len()
    }

    fn advance_char(&mut self) -> char {
        if self.source_at_end() {
            '\0'
        } else {
            self.pos.1 += 1;
            self.location.end.col += 1;
            self.chars[self.pos.1 - 1]
        }
    }

    fn peek_char(&self) -> char {
        if self.source_at_end() {
            '\0'
        } else {
            self.chars[self.pos.1]
        }
    }

    fn error(&mut self, msg: String) -> Result<(), LexicalError> {
        let err = Err(LexicalError {
            location: self.location.clone(),
            msg,
        });
        self.fresh_pos();
        err
    }
}

pub struct LexicalError {
    location: Location,
    msg: String,
}

impl LoxinasError for LexicalError {
    fn get_type() -> &'static str {
        "Lexical Error"
    }

    fn get_location(&self) -> &Location {
        &self.location
    }

    fn get_msg(&self) -> &str {
        &self.msg
    }
}
