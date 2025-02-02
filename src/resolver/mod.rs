//! 语义分析模块

use std::collections::HashMap;
use std::rc::Rc;

use crate::expr::Expr;
use crate::position::Position;
use crate::tokens::Token;
use crate::types::ValueType;

mod resolver_expr;

/// 语义分析器
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Resolver{
    pub expr_res_type: HashMap<*const Expr, ValueType>,
}

impl Resolver {
    pub fn new() -> Self {
        Self { expr_res_type: HashMap::new() }
    }
    
    /// 分析表达式
    pub fn resolve_expr(&mut self, expr: &Expr) -> Result<ResolverRes, CompileError> {
        let res: ResolverRes = expr.accept(self)?;
        let ptr: *const Expr = expr as *const Expr;
        self.expr_res_type.insert(ptr, res.expr_type.clone());
        return Ok(res);
    }

    /// 操作符转字符串，方便报错
    pub fn operator_to_string(token: &Rc<Token>) -> String {
        use crate::tokens::TokenType;
        String::from(
            match &token.token_type {
                TokenType::Operator(operator) => {
                    use crate::tokens::TokenOperator::*;
                    match operator {
                        Plus => "+",
                        Minus => "-",
                        Star => "*",
                        Slash => "/",
                        Power => "**",
                        Backslash => "\\",
                        And => "&",
                        Pipe => "|",
                        Tilde => "~",
                        Colon => ":",
                        Semicolon => ";",
                        Equal => "=",
                        EqualEqual => "==",
                        NotEqual => "!=",
                        Less => "<",
                        LessEqual => "<=",
                        Greater => ">",
                        GreaterEqual => ">=",
                        Bang => "!",
                        Caret => "^",
                        Mod => "%",
                    }
                }
                TokenType::Keyword(operator) => {
                    use crate::tokens::TokenKeyword::*;
                    match operator {
                        And => "and",
                        Or => "or",
                        Not => "not",
                        _ => panic!("Invalid operator"),
                    }
                }
                _ => panic!("Invalid operator"),
            }
        )
    }
}

/// 语义分析结果
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ResolverRes {
    /// 表达式得到的结果类型
    pub expr_type: ValueType,
}

impl ResolverRes {
    pub fn new(expr_type: ValueType) -> Self {
        Self { expr_type }
    }
}

/// 编译错误
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CompileError {
    /// 出错位置
    pub pos: Position,
    /// 错误信息
    pub message: String,
}

impl CompileError {
    pub fn new(pos: &Position, message: String) -> Self {
        Self { pos: pos.clone(), message }
    }
}
