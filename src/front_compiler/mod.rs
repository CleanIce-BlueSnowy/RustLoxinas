//! 前端编译器模块

mod front_compiler_expr;
mod front_compiler_stmt;
mod front_compiler_assistance;

use std::collections::LinkedList;
use crate::compiler::Compiler;
use crate::errors::error_types::CompileError;
use crate::instr::Instruction;
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
    #[must_use]
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
        let mut errors = vec![];
        let mut codes = LinkedList::new();
        
        self.resolver.enter_scope();
        
        self.compile_scope(&mut errors, &mut codes, self.statements);
        
        self.resolver.leave_scope();
        
        // 补充返回指令，临时充当结束程序的作用
        codes.push_back(Instruction::OpReturn.into());
        
        return if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(codes.into_iter().collect())
        };
    }
    
    /// 编译一个作用域
    pub fn compile_scope(&mut self, errors: &mut Vec<CompileError>, codes: &mut LinkedList<u8>, statements: &[Stmt]) {
        if let Err(mut errs) = self.resolver.predefine(statements) {
            errors.append(&mut errs);
        }

        for statement in statements {
            match statement.accept(self) {
                Err(mut err) => errors.append(&mut err),
                Ok(mut code) => codes.append(&mut code),
            }
        }
    }
}
