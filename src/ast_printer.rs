use crate::expr::Expr;
use crate::tokens::{TokenKeyword, TokenOperator, TokenType};

/// 打印表达式的抽象语法树，实现 Visitor<String> 特征
#[cfg(debug_assertions)]
#[derive(Debug)]
pub struct AstPrinter { }

#[cfg(debug_assertions)]
impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }

    /// 打印完整的语法树
    pub fn print(&mut self, expr: &Expr) -> String {
        return expr.accept(self);
    }

    /// 将合法的运算符转换为运算符对应的字符串
    pub fn operator_to_string(&self, token: &TokenType) -> String {
        match token {
            TokenType::Operator(TokenOperator::And) => "&",
            TokenType::Operator(TokenOperator::NotEqual) => "!=",
            TokenType::Operator(TokenOperator::Caret) => "~",
            TokenType::Operator(TokenOperator::Equal) => "=",
            TokenType::Operator(TokenOperator::EqualEqual) => "==",
            TokenType::Operator(TokenOperator::Greater) => ">",
            TokenType::Operator(TokenOperator::GreaterEqual) => ">=",
            TokenType::Operator(TokenOperator::Less) => "<",
            TokenType::Operator(TokenOperator::LessEqual) => "<=",
            TokenType::Operator(TokenOperator::Minus) => "-",
            TokenType::Operator(TokenOperator::Pipe) => "|",
            TokenType::Operator(TokenOperator::Plus) => "+",
            TokenType::Operator(TokenOperator::Power) => "**",
            TokenType::Operator(TokenOperator::Slash) => "/",
            TokenType::Operator(TokenOperator::Star) => "*",
            TokenType::Operator(TokenOperator::Tilde) => "~",
            TokenType::Keyword(TokenKeyword::And) => "and",
            TokenType::Keyword(TokenKeyword::Not) => "not",
            TokenType::Keyword(TokenKeyword::Or) => "or",
            _ => panic!("Invalid token: {token:?}"),  // 不合法的运算符令牌，在解析表达式时就应该去除
        }.to_string()
    }

    /// 为表达式添加括号
    pub fn parenthesize(&mut self, name: &str, exprs: &[&Box<Expr>]) -> String {
        let mut res = String::new();
        res.push('(');
        res.push_str(name);
        for expr in exprs {
            res.push(' ');
            let str: String = expr.accept(self);
            res.push_str(&str);
        }
        res.push(')');
        return res;
    }
}
