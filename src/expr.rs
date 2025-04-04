//! 表达式模块

use std::rc::Rc;

use crate::data::Data;
use crate::position::Position;
use crate::tokens::Token;
use crate::types::TypeTag;

/** 表达式

使用访问者模式
 */
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Expr {
    /// 二元操作
    Binary(Box<ExprBinary>),
    /// 分组，用于改变计算优先级，通过作为表达式生成树单独节点实现
    Grouping(Box<ExprGrouping>),
    /// 字面量
    Literal(Box<ExprLiteral>),
    /// 一元操作
    Unary(Box<ExprUnary>),
    /// 类型转换操作
    As(Box<ExprAs>),
    /// 变量
    Variable(Box<ExprVariable>),
}

/// 二元操作表达式
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprBinary {
    /// 位置信息
    pub pos: Position,
    /// 左操作数
    pub left: Expr,
    /// 操作符
    pub operator: Rc<Token>,
    /// 右操作数
    pub right: Expr,
}

/// 分组表达式
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprGrouping {
    /// 位置信息
    pub pos: Position,
    /// 组内的表达式
    pub expression: Expr,
}

/// 字面量表达式
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprLiteral {
    /// 位置信息
    pub pos: Position,
    /// 字面量的值
    pub value: Data,
}

/// 一元操作表达式
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprUnary {
    /// 位置信息
    pub pos: Position,
    /// 操作符
    pub operator: Rc<Token>,
    /// 操作数
    pub right: Expr,
}

/// 类型转换表达式
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprAs {
    /// 位置信息
    pub pos: Position,
    /// 操作数
    pub expression: Expr,
    /// 目标类型
    pub target: TypeTag,
}

/// 变量表达式
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprVariable {
    /// 位置信息
    pub pos: Position,
    /// 变量名称
    pub name: String,
}

/** 使用访问者模式的访问器，用于访问各种表达式，从而访问表达式抽象语法树

`RetType` 是返回类型
 */
pub trait ExprVisitor<RetType> {
    #[must_use]
    fn visit_binary_expr(&mut self, this: *const Expr, expr: &ExprBinary) -> RetType;
    #[must_use]
    fn visit_grouping_expr(&mut self, this: *const Expr, expr: &ExprGrouping) -> RetType;
    #[must_use]
    fn visit_literal_expr(&mut self, this: *const Expr, expr: &ExprLiteral) -> RetType;
    #[must_use]
    fn visit_unary_expr(&mut self, this: *const Expr, expr: &ExprUnary) -> RetType;
    #[must_use]
    fn visit_as_expr(&mut self, this: *const Expr, expr: &ExprAs) -> RetType;
    #[must_use]
    fn visit_variable_expr(&mut self, this: *const Expr, expr: &ExprVariable) -> RetType;
}

impl Expr {
    /// 访问自己，通过模式匹配具体的枚举值
    #[must_use]
    pub fn accept<RetType>(&self, visitor: &mut dyn ExprVisitor<RetType>) -> RetType {
        let ptr = self as *const Expr;
        return match self {
            Expr::Binary(expr) => visitor.visit_binary_expr(ptr, expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(ptr, expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(ptr, expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(ptr, expr),
            Expr::As(expr) => visitor.visit_as_expr(ptr, expr),
            Expr::Variable(expr) => visitor.visit_variable_expr(ptr, expr),
        };
    }
}

/// 获取表达式的位置信息
#[macro_export]
macro_rules! expr_get_pos {
    ( $expression:expr ) => {
        {
            use crate::expr::Expr;
            match $expression {
                Expr::Binary(expr) => expr.pos.clone(),
                Expr::Grouping(expr) => expr.pos.clone(),
                Expr::Literal(expr) => expr.pos.clone(),
                Expr::Unary(expr) => expr.pos.clone(),
                Expr::As(expr) => expr.pos.clone(),
                Expr::Variable(expr) => expr.pos.clone(), 
            }
        }
    }
}
