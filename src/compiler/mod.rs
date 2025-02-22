//! 编译器模块

use std::collections::LinkedList;

mod compiler_assistance;
mod compiler_expr;
mod compiler_stmt;

/// 编译器
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Compiler {
    pub temp_chunk: LinkedList<u8>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            temp_chunk: LinkedList::new(),
        }
    }
}
