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
    /// 调用函数
    Call(Box<ExprCall>),
}

/// 二元操作表达式
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprBinary {
    pub pos: Position,
    pub left: Expr,
    pub operator: Rc<Token>,
    pub right: Expr,
}

/// 分组表达式
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprGrouping {
    pub pos: Position,
    pub expression: Expr,
}

/// 字面量表达式
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprLiteral {
    pub pos: Position,
    pub value: Data,
}

/// 一元操作表达式
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprUnary {
    pub pos: Position,
    pub operator: Rc<Token>,
    pub right: Expr,
}

/// 类型转换表达式
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprAs {
    pub pos: Position,
    pub expression: Expr,
    pub target: TypeTag,
}

/// 变量表达式
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprVariable {
    pub pos: Position,
    pub name: String,
}

/// 函数调用表达式
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprCall {
    pub pos: Position,
    pub func_name: String,
    pub arguments: Vec<Expr>,
}

/** 使用访问者模式的访问器，用于访问各种表达式，从而访问表达式抽象语法树

`RetType` 是返回类型
 */
pub trait ExprVisitor<RetType> {
    #[must_use]
    fn visit_binary_expr(&mut self, expr: &ExprBinary) -> RetType;
    #[must_use]
    fn visit_grouping_expr(&mut self, expr: &ExprGrouping) -> RetType;
    #[must_use]
    fn visit_literal_expr(&mut self, expr: &ExprLiteral) -> RetType;
    #[must_use]
    fn visit_unary_expr(&mut self, expr: &ExprUnary) -> RetType;
    #[must_use]
    fn visit_as_expr(&mut self, expr: &ExprAs) -> RetType;
    #[must_use]
    fn visit_variable_expr(&mut self, expr: &ExprVariable) -> RetType;
    #[must_use]
    fn visit_call_expr(&mut self, expr: &ExprCall) -> RetType;
}

impl Expr {
    /// 访问自己，通过模式匹配具体的枚举值
    #[must_use]
    pub fn accept<RetType>(&self, visitor: &mut impl ExprVisitor<RetType>) -> RetType {
        match self {
            Self::Binary(expr) => visitor.visit_binary_expr(expr),
            Self::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Self::Literal(expr) => visitor.visit_literal_expr(expr),
            Self::Unary(expr) => visitor.visit_unary_expr(expr),
            Self::As(expr) => visitor.visit_as_expr(expr),
            Self::Variable(expr) => visitor.visit_variable_expr(expr),
            Self::Call(expr) => visitor.visit_call_expr(expr),
        }
    }
}

/// 获取表达式的位置信息
#[macro_export]
macro_rules! expr_get_pos {
    ( $expression:expr ) => {{
        use crate::expr::Expr;
        match $expression {
            Expr::Binary(expr) => expr.pos.clone(),
            Expr::Grouping(expr) => expr.pos.clone(),
            Expr::Literal(expr) => expr.pos.clone(),
            Expr::Unary(expr) => expr.pos.clone(),
            Expr::As(expr) => expr.pos.clone(),
            Expr::Variable(expr) => expr.pos.clone(),
            Expr::Call(expr) => expr.pos.clone(),
        }
    }};
}
