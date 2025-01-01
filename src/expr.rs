use crate::data::Data;
use crate::tokens::Token;

/// 表达式
/// 
/// 使用访问者模式
#[allow(dead_code)]
#[derive(Debug)]
pub enum Expr {
    /// 二元操作
    Binary {
        /// 左操作数
        left: Box<Expr>,
        /// 操作符
        operator: Token,
        /// 右操作数
        right: Box<Expr>,
    },
    /// 分组，用于改变计算优先级，通过作为表达式生成树单独节点实现
    Grouping {
        /// 组内的表达式
        expression: Box<Expr>,
    },
    /// 字面量
    Literal {
        /// 字面量的值
        value: Data,
    },
    /// 一元操作
    Unary {
        /// 操作符
        operator: Token,
        /// 操作数
        right: Box<Expr>,
    },
}

/// 使用访问者模式的访问器，用于访问各种表达式，从而访问表达式生成树
/// 
/// `RetType` 是返回类型
/// 
/// **注意，每一个方法都应该检查 `expr` 的枚举值是否正确，但应该使用断言**
/// 
/// **因为理论上枚举值都是正确的，若不正确说明代码存在问题**
#[allow(dead_code)]
pub trait Visitor<RetType> {
    /// 访问二元操作
    fn visit_binary_expr(&mut self, expr: &Expr) -> RetType;
    /// 访问分组
    fn visit_grouping_expr(&mut self, expr: &Expr) -> RetType;
    /// 访问字面量
    fn visit_literal_expr(&mut self, expr: &Expr) -> RetType;
    /// 访问一元操作
    fn visit_unary_expr(&mut self, expr: &Expr) -> RetType;
}

impl Expr {
    /// 访问自己，通过模式匹配具体的枚举值
    pub fn accept<RetType>(&self, visitor: &mut dyn Visitor<RetType>) -> RetType {
        match self {
            Expr::Binary{ left: _, operator: _, right: _ } => visitor.visit_binary_expr(self),
            Expr::Grouping{ expression: _ } => visitor.visit_grouping_expr(self),
            Expr::Literal{ value: _ } => visitor.visit_literal_expr(self),
            Expr::Unary{ operator: _, right: _ } => visitor.visit_unary_expr(self),
        }
    }
}
