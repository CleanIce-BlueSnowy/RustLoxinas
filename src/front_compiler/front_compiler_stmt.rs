use std::collections::LinkedList;
use crate::front_compiler::FrontCompiler;
use crate::errors::error_types::CompileError;
use crate::expr_get_pos;
use crate::stmt::{Stmt, StmtAssign, StmtExpr, StmtInit, StmtLet, StmtPrint, StmtVisitor};

impl<'a> StmtVisitor<Result<LinkedList<u8>, CompileError>> for FrontCompiler<'a> {
    fn visit_expr_stmt(&mut self, _this: *const Stmt, stmt: &StmtExpr) -> Result<LinkedList<u8>, CompileError> {
        let (expr_res, mut expr_code) = stmt.expression.accept(self)?;
        self.resolver.resolve_expr_stmt()?;
        let final_code = self.compiler.compile_expr_stmt(&mut expr_code, &expr_res)?;
        return Ok(final_code);
    }

    fn visit_let_stmt(&mut self, _this: *const Stmt, stmt: &StmtLet) -> Result<LinkedList<u8>, CompileError> {
        self.in_ref_let = stmt.is_ref;
        let (init_res, mut init_code) = if let Some(init) = &stmt.init {
            let (a, b) = init.accept(self)?;
            (Some(a), Some(b))
        } else {
            (None, None)
        };
        let var_type = self.resolver.resolve_let_stmt(stmt, init_res.as_ref())?;
        let final_code = self.compiler.compile_let_stmt(init_code.as_mut(), init_res.as_ref(), var_type)?;
        self.in_ref_let = false;
        return Ok(final_code);
    }

    fn visit_init_stmt(&mut self, _this: *const Stmt, stmt: &StmtInit) -> Result<LinkedList<u8>, CompileError> {
        let (init_res, mut init_code) = stmt.init.accept(self)?;
        let (var_type, slot) = self.resolver.resolve_init_stmt(stmt, &init_res)?;
        let final_code = self.compiler.compile_init_stmt(slot, &mut init_code, &init_res, var_type)?;
        return Ok(final_code);
    }

    fn visit_assign_stmt(&mut self, _this: *const Stmt, stmt: &StmtAssign) -> Result<LinkedList<u8>, CompileError> {
        self.in_assign = true;
        let mut vars_res = Vec::with_capacity(stmt.assign_vars.len());
        let mut vars_code = Vec::with_capacity(stmt.assign_vars.len());
        for var in &stmt.assign_vars {
            let (var_res, var_code) = var.accept(self)?;
            vars_res.push(var_res);
            vars_code.push(var_code);
        }
        let (right_res, mut right_code) = stmt.right_expr.accept(self)?;
        self.resolver.resolve_assign_stmt(stmt, &vars_res, &right_res)?;
        let final_code = self.compiler.compile_assign_stmt(&mut vars_code, &vars_res, &mut right_code, &right_res)?;
        self.in_assign = false;
        return Ok(final_code);
    }

    fn visit_print_stmt(&mut self, _this: *const Stmt, stmt: &StmtPrint) -> Result<LinkedList<u8>, CompileError> {
        let (expr_res, expr_code, expr_pos) = if let Some(expr) = &stmt.expr {
            let (res, code) = expr.accept(self)?;
            (Some(res), Some(code), Some(expr_get_pos!(expr.as_ref())))
        } else {
            (None, None, None)
        };
        self.resolver.resolve_print_stmt()?;
        let final_code = self.compiler.compile_print_stmt(expr_code, expr_res, expr_pos)?;
        return Ok(final_code);
    }
}
