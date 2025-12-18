use crate::compiler::ast::Operator;
use crate::location::Location;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Expr {
    pub location: Location,
    pub expr_type: Box<ExprType>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum ExprType {
    Binary(ExprBinary),
    Unary(ExprUnary),
    Variable(ExprVariable),
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprBinary {
    pub ope: Operator,
    pub lhs: Expr,
    pub rhs: Expr,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprUnary {
    pub ope: Operator,
    pub rhs: Expr,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprVariable {
    pub name: String,
}
