//! 编译器模块

use std::collections::HashMap;

use crate::expr::Expr;
use crate::types::ValueType;

mod compiler_assistance;
mod compiler_expr;

/// 编译器
pub struct Compiler {
    pub const_pool: Vec<u8>,
    pub chunk: Vec<u8>,
    expr_res_type: HashMap<*const Expr, ValueType>,
}

impl Compiler {
    pub fn new(expr_res_type: HashMap<*const Expr, ValueType>) -> Self {
        Self { const_pool: Vec::new(), chunk: Vec::new(), expr_res_type }
    }
    
    pub fn compile_expression(&mut self, expr: &Expr) {
        expr.accept(self);
    }
}
