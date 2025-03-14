use std::collections::LinkedList;
use crate::front_compiler::FrontCompiler;
use crate::errors::error_types::CompileError;
use crate::stmt::{Stmt, StmtExpr, StmtInit, StmtLet, StmtVisitor};

impl<'a> StmtVisitor<Result<LinkedList<u8>, CompileError>> for FrontCompiler<'a> {
    fn visit_expr_stmt(&mut self, _this: *const Stmt, stmt: &StmtExpr) -> Result<LinkedList<u8>, CompileError> {
        let (expr_res, mut expr_code) = stmt.expression.accept(self)?;
        self.resolver.resolve_expr_stmt()?;
        let final_code = self.compiler.compile_expr_stmt(&expr_res, &mut expr_code)?;
        return Ok(final_code);
    }

    fn visit_let_stmt(&mut self, _this: *const Stmt, stmt: &StmtLet) -> Result<LinkedList<u8>, CompileError> {
        let (init_res, mut init_code) = if let Some(init) = &stmt.init {
            let (a, b) = init.accept(self)?;
            (Some(a), Some(b))
        } else {
            (None, None)
        };
        let var_type = self.resolver.resolve_let_stmt(stmt, init_res.as_ref())?;
        let final_code = self.compiler.compile_let_stmt(init_code.as_mut(), init_res.as_ref(), var_type)?;
        return Ok(final_code);
    }

    fn visit_init_stmt(&mut self, _this: *const Stmt, stmt: &StmtInit) -> Result<LinkedList<u8>, CompileError> {
        let (init_res, mut init_code) = stmt.init.accept(self)?;
        let (var_type, slot) = self.resolver.resolve_init_stmt(stmt, &init_res)?;
        let final_code = self.compiler.compile_init_stmt(slot, &mut init_code, &init_res, var_type)?;
        return Ok(final_code);
    }
}
