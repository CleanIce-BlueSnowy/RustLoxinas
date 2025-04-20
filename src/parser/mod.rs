//! 语法分析模块

use crate::errors::error_types::{SyntaxError, SyntaxResultList};
use crate::stmt::Stmt;
use crate::tokens::Token;
use std::rc::Rc;

mod parser_assistance;
mod parser_expr;
mod parser_stmt;

/// 语法分析器，生成 AST
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Parser {
    /// 令牌列表
    pub tokens: Vec<Rc<Token>>,
    /// 当前令牌
    pub current: usize,
    /// 在迭代循环语句的更新语句中
    pub in_for_update: bool,
}

impl Parser {
    /// 构造函数，将移动 `tokens`
    #[must_use]
    pub fn new(tokens: Vec<Rc<Token>>) -> Self {
        Self {
            tokens,
            current: 0,
            in_for_update: false,
        }
    }

    /// 解析
    #[inline]
    pub fn parse(&mut self) -> SyntaxResultList<Vec<Stmt>> {
        let mut errors: Vec<SyntaxError> = vec![];
        let mut statements: Vec<Stmt> = vec![];

        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    errors.push(err);
                    self.synchronize();
                }
            }
        }

        return if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        };
    }
}
