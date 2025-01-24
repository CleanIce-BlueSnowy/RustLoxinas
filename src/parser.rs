use std::rc::Rc;

use crate::expr::Expr;
use crate::tokens::Token;

/// 语法分析器，生成 AST
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
pub struct SyntaxError {
    /// 错误位置令牌
    pub token: Rc<Token>,
    /// 错误信息
    pub message: String,
}
