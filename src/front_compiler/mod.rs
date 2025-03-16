//! 前端编译器模块

mod front_compiler_expr;
mod front_compiler_stmt;

use std::collections::LinkedList;
use crate::compiler::Compiler;
use crate::errors::error_types::CompileError;
use crate::resolver::Resolver;
use crate::stmt::Stmt;

/// 前端编译器
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct FrontCompiler<'a> {
    pub resolver: Resolver,
    pub compiler: Compiler,
    statements: &'a [Stmt],
    in_assign: bool,
    in_ref_let: bool,
}

impl<'a> FrontCompiler<'a> {
    pub fn new(statements: &'a [Stmt]) -> Self {
        Self {
            resolver: Resolver::new(), 
            compiler: Compiler::new(), 
            statements, 
            in_assign: false,
            in_ref_let: false,
        }
    }
    
    /// 启动编译
    pub fn compile(&mut self) -> Result<Vec<u8>, Vec<CompileError>> {
        let mut errors = Vec::new();
        let mut codes = LinkedList::new();
        
        self.resolver.enter_scope();
        if let Err(mut errs) = self.resolver.predefine(self.statements) {
            errors.append(&mut errs);
        }
        
        for statement in self.statements {
            match statement.accept(self) {
                Err(err) => errors.push(err),
                Ok(mut code) => codes.append(&mut code),
            }
        }
        
        self.resolver.leave_scope();
        
        return if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(codes.into_iter().collect())
        };
    }
}
