//! 语法树打印模块

use indexmap::IndexMap;

use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::tokens::{TokenKeyword, TokenOperator, TokenType};

mod ast_printer_expr;
mod ast_printer_stmt;

/// 语法树子树节点
pub enum TreeChild<'a> {
    Expr(&'a Expr),
    Stmt(&'a Stmt),
    StmtList(&'a [Stmt]),
    ExprList(&'a [Expr]),
    Identifier(&'a str),
    Tag(&'a str),
}

/// 打印表达式的抽象语法树
#[cfg(debug_assertions)]
#[derive(Debug)]
pub struct AstPrinter { }

#[cfg(debug_assertions)]
impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }

    /// 打印完整的语法树
    pub fn print(&mut self, statements: &[Stmt]) -> String {
        let mut result = String::new();
        for statement in statements {
            let one: String = statement.accept(self);
            result.push_str(&format!("{}\n", one));
        }
        return result;
    }

    /// 将合法的运算符转换为运算符对应的字符串
    pub fn operator_to_string(&self, token: &TokenType) -> String {
        use crate::tokens::TokenType::*;
        use crate::tokens::TokenOperator::*;
        use crate::tokens::TokenKeyword::*;
        
        match token {
            Operator(TokenOperator::And) => "&",
            Operator(NotEqual) => "!=",
            Operator(Caret) => "^",
            Operator(Equal) => "=",
            Operator(EqualEqual) => "==",
            Operator(Greater) => ">",
            Operator(GreaterEqual) => ">=",
            Operator(Less) => "<",
            Operator(LessEqual) => "<=",
            Operator(Minus) => "-",
            Operator(Pipe) => "|",
            Operator(Plus) => "+",
            Operator(Power) => "**",
            Operator(Slash) => "/",
            Operator(Star) => "*",
            Operator(Tilde) => "~",
            Operator(Mod) => "%",
            Keyword(TokenKeyword::And) => "and",
            Keyword(Not) => "not",
            Keyword(Or) => "or",
            Keyword(Shl) => "shl",
            Keyword(Shr) => "shr",
            _ => panic!("Invalid token: {token:?}"),  // 不合法的运算符令牌，在解析表达式时就应该去除
        }.to_string()
    }

    /// 为语法树节点添加括号并格式化
    pub fn parenthesize(&mut self, name: &str, exprs: IndexMap<&str, TreeChild>) -> String {
        let mut res = String::new();
        res.push_str("( ");
        res.push_str(name);
        res.push('\n');
        for (name, child) in exprs {
            let str: String = match child {
                TreeChild::Expr(expr) => expr.accept(self),
                TreeChild::Stmt(stmt) => stmt.accept(self),
                TreeChild::StmtList(list) => {
                    let mut res = String::new();
                    for stmt in list {
                        res.push_str(&format!("{}, ", stmt.accept(self)));
                    }
                    res
                }
                TreeChild::ExprList(list) => {
                    let mut res = String::new();
                    for expr in list {
                        res.push_str(&format!("{}, ", expr.accept(self)));
                    }
                    res
                }
                TreeChild::Identifier(identifier) => identifier.to_string(),
                TreeChild::Tag(tag_name) => format!("@{}", tag_name),
            };
            res.push_str(&format!("    {name}: "));
            let mut first_line = true;
            for line in str.split('\n') {
                if first_line {
                    first_line = false;
                } else {
                    res.push_str("    ");
                }
                res.push_str(&format!("{}\n", line));
            }
        }
        res.push(')');
        return res;
    }
}
