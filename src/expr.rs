//! 表达式模块

use std::rc::Rc;
use crate::data::Data;
use crate::position::Position;
use crate::tokens::Token;

/** 表达式

使用访问者模式
 */
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Expr {
    /// 二元操作
    Binary {
        /// 位置信息
        pos: Position,
        /// 左操作数
        left: Box<Expr>,
        /// 操作符
        operator: Rc<Token>,
        /// 右操作数
        right: Box<Expr>,
    },
    /// 分组，用于改变计算优先级，通过作为表达式生成树单独节点实现
    Grouping {
        /// 位置信息
        pos: Position,
        /// 组内的表达式
        expression: Box<Expr>,
    },
    /// 字面量
    Literal {
        /// 位置信息
        pos: Position,
        /// 字面量的值
        value: Data,
    },
    /// 一元操作
    Unary {
        /// 位置信息
        pos: Position,
        /// 操作符
        operator: Rc<Token>,
        /// 操作数
        right: Box<Expr>,
    },
}

/** 使用访问者模式的访问器，用于访问各种表达式，从而访问表达式抽象语法树

`RetType` 是返回类型
 */
pub trait ExprVisitor<RetType> {
    /// 访问二元操作
    fn visit_binary_expr(&mut self, this: &Expr, pos: &Position, left: &Box<Expr>, operator: &Rc<Token>, right: &Box<Expr>) -> RetType;
    /// 访问分组
    fn visit_grouping_expr(&mut self, this: &Expr, pos: &Position, expr: &Box<Expr>) -> RetType;
    /// 访问字面量
    fn visit_literal_expr(&mut self, this: &Expr, pos: &Position, value: &Data) -> RetType;
    /// 访问一元操作
    fn visit_unary_expr(&mut self, this: &Expr, pos: &Position, operator: &Rc<Token>, right: &Box<Expr>) -> RetType;
}

impl Expr {
    /// 访问自己，通过模式匹配具体的枚举值
    pub fn accept<RetType>(&self, visitor: &mut dyn ExprVisitor<RetType>) -> RetType {
        match self {
            Expr::Binary { pos, left, operator, right } => visitor.visit_binary_expr(self, &pos, &left, &operator, &right),
            Expr::Grouping { pos, expression } => visitor.visit_grouping_expr(self, &pos, &expression),
            Expr::Literal { pos, value } => visitor.visit_literal_expr(self, &pos, &value),
            Expr::Unary { pos, operator, right } => visitor.visit_unary_expr(self, &pos, &operator, &right),
        }
    }
}

#[macro_export]
macro_rules! expr_get_pos {
    ( $expr:expr ) => {
        match $expr {
            Expr::Binary { pos, left: _, operator: _, right: _ } => pos.clone(),
            Expr::Grouping { pos, expression: _ } => pos.clone(),
            Expr::Literal { pos, value: _ } => pos.clone(),
            Expr::Unary { pos, operator: _, right: _ } => pos.clone(),
        }
    }
}
