use std::rc::Rc;
use crate::expr::Expr;
use crate::tokens::Token;
use crate::types::ValueType;

/// 语义分析器
pub struct Resolver {

}

impl Resolver {
    pub fn new() -> Self {
        Self {}
    }
    
    /// 分析表达式
    pub fn resolve_expr(&mut self, expr: &Expr) -> Result<ResolverRes, CompileError> {
        expr.accept(self)
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
pub struct CompileError {
    /// 出错位置的令牌
    pub token: Rc<Token>,
    /// 错误信息
    pub message: String,
}

impl CompileError {
    pub fn new(token: Rc<Token>, message: String) -> Self {
        Self { token, message }
    }
}
