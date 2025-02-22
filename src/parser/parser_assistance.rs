//! 语法分析——辅助功能模块

use std::rc::Rc;
use crate::errors::error_types::SyntaxError;
use crate::parser::Parser;
use crate::position::Position;
use crate::tokens::{Token, TokenOperator, TokenType};
use crate::types::TypeTag;

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
    #[inline]
    pub fn is_at_end(&self) -> bool {
        return matches!(self.peek().token_type, TokenType::EOF);
    }

    /// 下一个令牌，不消耗
    #[inline]
    pub fn peek(&self) -> Rc<Token> {
        return Rc::clone(&self.tokens[self.current]);
    }

    /// 当前令牌
    #[inline]
    pub fn previous(&self) -> Rc<Token> {
        return Rc::clone(&self.tokens[self.current - 1]);
    }

    /// 消耗令牌并返回下一个令牌
    #[inline]
    pub fn advance(&mut self) -> Rc<Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    /// 解析类型标识符
    pub fn parse_type_tag(&mut self) -> Result<TypeTag, SyntaxError> {
        use crate::tokens::TokenType::*;
        use crate::tokens::TokenOperator::*;
        let next_token = self.advance().clone();
        return if let Identifier(name) = &next_token.token_type {
            let mut tag = TypeTag::new();
            tag.pos.start_line = next_token.line;
            tag.pos.start_idx = next_token.start;
            tag.pos.end_line = next_token.line;
            tag.pos.end_idx = next_token.end;
            tag.chain.push_back(name.clone());
            while parser_can_match!(self, Operator(DoubleColon)) {
                let token = self.advance().clone();
                if let Identifier(name) = &token.token_type {
                    tag.pos.end_line = token.line;
                    tag.pos.end_idx = token.end;
                    tag.chain.push_back(name.clone());
                } else {
                    return Err(SyntaxError::new(
                        &Position::new(token.line, token.start, token.line, token.end),
                        "Expect type name".to_string()
                    ));
                }
            }
            Ok(tag)
        } else {
            Err(SyntaxError::new(
                &Position::new(next_token.line, next_token.start, next_token.line, next_token.end),
                "Expect type name".to_string()
            ))
        }
    }
    
    /// 同步错误
    pub fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if let TokenType::Operator(TokenOperator::Semicolon) = &self.previous().token_type {
                return;
            } else if let TokenType::Keyword(keyword) = &self.peek().token_type {
                use crate::tokens::TokenKeyword::*;
                if let If | Elif | Else | For | While | Let = keyword {
                    return;
                }
            }
            self.advance();
        }
    }
}
