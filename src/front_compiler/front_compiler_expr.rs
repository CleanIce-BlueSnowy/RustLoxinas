use std::collections::LinkedList;
use crate::errors::error_types::CompileError;
use crate::expr::{Expr, ExprAs, ExprBinary, ExprGrouping, ExprLiteral, ExprUnary, ExprVariable, ExprVisitor};
use crate::front_compiler::FrontCompiler;
use crate::resolver::ExprResolveRes;

impl<'a> ExprVisitor<Result<(ExprResolveRes, LinkedList<u8>), CompileError>> for FrontCompiler<'a> {
    fn visit_binary_expr(&mut self, _this: *const Expr, expr: &ExprBinary) -> Result<(ExprResolveRes, LinkedList<u8>), CompileError> {
        let (left_res, mut left_code) = expr.left.accept(self)?;
        let (right_res, mut right_code) = expr.right.accept(self)?;
        let expr_res = self.resolver.resolve_binary_expr(expr, &left_res, &right_res)?;
        let expr_code = self.compiler.compile_binary_expr(expr, &expr_res, &left_res, &right_res, &mut left_code, &mut right_code)?;
        return Ok((expr_res, expr_code));
    }

    fn visit_grouping_expr(&mut self, _this: *const Expr, expr: &ExprGrouping) -> Result<(ExprResolveRes, LinkedList<u8>), CompileError> {
        let (inside_res, mut inside_code) = expr.expression.accept(self)?;
        let expr_res = self.resolver.resolve_grouping_expr(&inside_res)?;
        let expr_code = self.compiler.compile_grouping_expr(&mut inside_code)?;
        return Ok((expr_res, expr_code));
    }

    fn visit_literal_expr(&mut self, _this: *const Expr, expr: &ExprLiteral) -> Result<(ExprResolveRes, LinkedList<u8>), CompileError> {
        let expr_res = self.resolver.resolve_literal_expr(expr)?;
        let expr_code = self.compiler.compile_literal_expr(expr)?;
        return Ok((expr_res, expr_code));
    }

    fn visit_unary_expr(&mut self, _this: *const Expr, expr: &ExprUnary) -> Result<(ExprResolveRes, LinkedList<u8>), CompileError> {
        let (right_res, mut right_code) = expr.right.accept(self)?;
        let expr_res = self.resolver.resolve_unary_expr(expr, &right_res)?;
        let expr_code = self.compiler.compile_unary_expr(expr, &right_res, &mut right_code)?;
        return Ok((expr_res, expr_code));
    }

    fn visit_as_expr(&mut self, _this: *const Expr, expr: &ExprAs) -> Result<(ExprResolveRes, LinkedList<u8>), CompileError> {
        let (inside_res, mut inside_code) = expr.expression.accept(self)?;
        let expr_res = self.resolver.resolve_as_expr(expr, &inside_res)?;
        let expr_code = self.compiler.compile_as_expr(&expr_res, &mut inside_code)?;
        return Ok((expr_res, expr_code));
    }

    fn visit_variable_expr(&mut self, _this: *const Expr, expr: &ExprVariable) -> Result<(ExprResolveRes, LinkedList<u8>), CompileError> {
        let (expr_res, slot) = self.resolver.resolve_variable_expr(expr)?;
        let expr_code = self.compiler.compile_variable_expr(&expr_res, slot)?;
        return Ok((expr_res, expr_code));
    }
}
