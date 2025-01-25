use std::rc::Rc;

use crate::parser::Parser;
use crate::tokens::{Token, TokenType};

/// 若下一个令牌能匹配模式，则消耗该令牌，并返回是否匹配
#[macro_export]
macro_rules! parser_can_match {
    ( $self:expr, $token_type:pat ) => {
        if parser_check!($self, $token_type) {
            $self.advance();
            true
        } else {
            false
        }
    }
}

/// 返回下一个令牌是否匹配模式
#[macro_export]
macro_rules! parser_check {
    ( $self:expr, $token_type:pat ) => {
        matches!(&$self.peek().token_type, $token_type)
    }
}

/// 若令牌匹配，则消耗令牌；若不匹配，则报错
#[macro_export]
macro_rules! parser_consume {
    ( $self:expr, $token_type:pat, $pos:expr, $message:expr ) => {
        if parser_check!($self, $token_type) {
            $self.advance();
            Ok(())
        } else {
            Err(SyntaxError::new($pos, $message))
        }
    }
}

impl Parser {
    /// 是否已到结尾
    pub fn is_at_end(&self) -> bool {
        return matches!(self.peek().token_type, TokenType::EOF);
    }

    /// 下一个令牌，不消耗
    pub fn peek(&self) -> Rc<Token> {
        return Rc::clone(&self.tokens[self.current]);
    }

    /// 当前令牌
    pub fn previous(&self) -> Rc<Token> {
        return Rc::clone(&self.tokens[self.current - 1]);
    }

    /// 消耗令牌并返回下一个令牌
    pub fn advance(&mut self) -> Rc<Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }
}
