use std::rc::Rc;
use crate::data::{Data, DataFloat, DataInteger};
use crate::expr::{Expr, Visitor};
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
    fn operator_to_string(&self, token: &TokenType) -> String {
        match token {
            TokenType::Operator(TokenOperator::And) => "&",
            TokenType::Operator(TokenOperator::BangEqual) => "!=",
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
    fn parenthesize(&mut self, name: &str, exprs: &[&Box<Expr>]) -> String {
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

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, left: &Box<Expr>, operator: &Rc<Token>, right: &Box<Expr>) -> String {
        let name = self.operator_to_string(&operator.token_type);
        return self.parenthesize(&name, &[left, right]);
    }

    fn visit_grouping_expr(&mut self, expr: &Box<Expr>) -> String {
        return self.parenthesize("group", &[expr]);
    }

    fn visit_literal_expr(&mut self, value: &Data) -> String {
        return match value {  // 将数据转换为对应的字符串，并带上 Loxinas 代码对应的数据后缀，字符串需要处理
            Data::Bool(res) => res.to_string(),
            Data::String(res) => {  // 处理字符串
                let mut ret = String::new();
                ret.push('"');
                for ch in res.clone().chars() {
                    match ch {
                        '"' => ret.push_str(r#"\""#),
                        '\n' => ret.push_str(r"\n"),
                        '\0' => ret.push_str(r"\0"),
                        '\t' => ret.push_str(r"\t"),
                        '\r' => ret.push_str(r"\r"),
                        '\\' => ret.push_str(r"\"),
                        '\'' => ret.push_str(r"'"),
                        _ => ret.push(ch),
                    }
                }
                ret
            }
            Data::Float(float) => {
                match float {
                    DataFloat::Float(res) => format!("{}f", res.to_string()),
                    DataFloat::Double(res) => res.to_string(),
                }
            }
            Data::Integer(integer) => {
                match integer {
                    DataInteger::Byte(res) => format!("{}b", res.to_string()),
                    DataInteger::SByte(res) => format!("{}sb", res.to_string()),
                    DataInteger::Short(res) => format!("{}s", res.to_string()),
                    DataInteger::UShort(res) => format!("{}us", res.to_string()),
                    DataInteger::Int(res) => res.to_string(),
                    DataInteger::UInt(res) => format!("{}u", res.to_string()),
                    DataInteger::Long(res) => format!("{}l", res.to_string()),
                    DataInteger::ULong(res) => format!("{}ul", res.to_string()),
                    DataInteger::ExtInt(res) => format!("{}e", res.to_string()),
                    DataInteger::UExtInt(res) => format!("{}ue", res.to_string()),
                }
            }
            Data::Char(ch) => {
                format!("'{}'", match ch {
                    '"' => r#"""#.to_string(),
                    '\n' => r"\n".to_string(),
                    '\0' => r"\0".to_string(),
                    '\t' => r"\t".to_string(),
                    '\r' => r"\r".to_string(),
                    '\\' => r"\\".to_string(),
                    '\'' => r"\'".to_string(),
                    _ => ch.to_string(),
                })
            }
        }
    }

    fn visit_unary_expr(&mut self, operator: &Rc<Token>, right: &Box<Expr>) -> String {
        let name = self.operator_to_string(&operator.token_type);
        return self.parenthesize(&name, &[right]);
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
