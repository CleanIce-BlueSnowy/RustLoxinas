use std::rc::Rc;
use crate::data::Data;
use crate::tokens::Token;

/// 表达式
/// 
/// 使用访问者模式
#[derive(Debug)]
pub enum Expr {
    /// 二元操作
    Binary {
        /// 左操作数
        left: Box<Expr>,
        /// 操作符
        operator: Rc<Token>,
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
        operator: Rc<Token>,
        /// 操作数
        right: Box<Expr>,
    },
}

/// 使用访问者模式的访问器，用于访问各种表达式，从而访问表达式抽象语法树
/// 
/// `RetType` 是返回类型
/// 
/// **注意，每一个方法都应该检查 `expr` 的枚举值是否正确**
/// 
/// **但是理论上枚举值都是正确的，若不正确说明代码存在问题**
pub trait ExprVisitor<RetType> {
    /// 访问二元操作
    fn visit_binary_expr(&mut self, left: &Box<Expr>, operator: &Rc<Token>, right: &Box<Expr>) -> RetType;
    /// 访问分组
    fn visit_grouping_expr(&mut self, expr: &Box<Expr>) -> RetType;
    /// 访问字面量
    fn visit_literal_expr(&mut self, value: &Data) -> RetType;
    /// 访问一元操作
    fn visit_unary_expr(&mut self, operator: &Rc<Token>, right: &Box<Expr>) -> RetType;
}

impl Expr {
    /// 访问自己，通过模式匹配具体的枚举值
    pub fn accept<RetType>(&self, visitor: &mut dyn ExprVisitor<RetType>) -> RetType {
        match self {
            Expr::Binary{ left, operator, right } => visitor.visit_binary_expr(&left, &operator, &right),
            Expr::Grouping{ expression } => visitor.visit_grouping_expr(&expression),
            Expr::Literal{ value } => visitor.visit_literal_expr(&value),
            Expr::Unary{ operator, right } => visitor.visit_unary_expr(&operator, &right),
        }
    }
}
