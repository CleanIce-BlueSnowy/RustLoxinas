use std::rc::Rc;

use crate::data::{Data, DataFloat, DataInteger};
use crate::expr::Expr;
use crate::tokens::{Token, TokenKeyword, TokenOperator, TokenType};

/// 打印表达式的抽象语法树，实现 Visitor<String> 特征
pub struct AstPrinter { }

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

#[cfg(debug_assertions)]
pub fn test() {
    // 1 + 2 / 3
    let test1 = Expr::Binary {
        left: Box::new(Expr::Literal { value: Data::Integer(DataInteger::Int(1)) }),
        operator: Rc::new(Token::new(TokenType::Operator(TokenOperator::Plus), 0, 0, 0)),
        right: Box::new(Expr::Binary {
            left: Box::new(Expr::Literal { value: Data::Integer(DataInteger::Int(2)) }),
            operator: Rc::new(Token::new(TokenType::Operator(TokenOperator::Slash), 0, 0, 0)),
            right: Box::new(Expr::Literal { value: Data::Integer(DataInteger::Int(3)) }),
        }),
    };
    // (-23s + 10u ** 2e) / (12.0f - 13 ^ 16)
    let test2 = Expr::Binary {
        left: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Binary {
                left: Box::new(Expr::Unary {
                    operator: Rc::new(Token::new(TokenType::Operator(TokenOperator::Minus), 0, 0, 0)),
                    right: Box::new(Expr::Literal { value: Data::Integer(DataInteger::Short(23)) }),
                }),
                operator: Rc::new(Token::new(TokenType::Operator(TokenOperator::Plus), 0, 0, 0)),
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal { value: Data::Integer(DataInteger::UInt(10)) }),
                    operator: Rc::new(Token::new(TokenType::Operator(TokenOperator::Power), 0, 0, 0)),
                    right: Box::new(Expr::Literal { value: Data::Integer(DataInteger::ExtInt(2)) }),
                }),
            }),
        }),
        operator: Rc::new(Token::new(TokenType::Operator(TokenOperator::Slash), 0, 0, 0)),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal { value: Data::Float(DataFloat::Float(12.0)) }),
                operator: Rc::new(Token::new(TokenType::Operator(TokenOperator::Minus), 0, 0, 0)),
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal { value: Data::Integer(DataInteger::Int(13)) }),
                    operator: Rc::new(Token::new(TokenType::Operator(TokenOperator::Caret), 0, 0, 0)),
                    right: Box::new(Expr::Literal { value: Data::Integer(DataInteger::Int(16)) }),
                }),
            }),
        }),
    };
    let mut ast_printer = AstPrinter::new();
    println!("AstPrinter Test 1: {}", ast_printer.print(&test1));
    println!("AstPrinter Test 2: {}", ast_printer.print(&test2));
}
