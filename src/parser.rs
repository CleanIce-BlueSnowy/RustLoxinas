use std::rc::Rc;

use crate::expr::Expr;
use crate::position::Position;
use crate::tokens::Token;

/// 语法分析器，生成 AST
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Parser {
    /// 令牌列表
    pub tokens: Vec<Rc<Token>>,
    /// 当前令牌
    pub current: usize,
}

impl Parser {
    /// 构造函数，将移动 `tokens`
    pub fn new(tokens: Vec<Rc<Token>>) -> Self {
        Self { tokens, current: 0 }
    }

    /// 解析
    pub fn parse(&mut self) -> Result<Expr, SyntaxError> {
        self.expression()
    }
}

/// 语法错误
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct SyntaxError {
    /// 错误位置
    pub pos: Position,
    /// 错误信息
    pub message: String,
}

impl SyntaxError {
    pub fn new(pos: &Position, message: String) -> Self {
        Self { pos: pos.clone(), message }
    }
}
