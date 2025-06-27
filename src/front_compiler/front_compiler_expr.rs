use crate::errors::error_types::{CompileError, CompileResultList};
use crate::expr::{ExprAs, ExprBinary, ExprCall, ExprGrouping, ExprLiteral, ExprUnary, ExprVariable, ExprVisitor};
use crate::front_compiler::FrontCompiler;
use crate::resolver::ExprResolveRes;

impl<'a> ExprVisitor<CompileResultList<(ExprResolveRes, Vec<u8>)>> for FrontCompiler<'a> {
    fn visit_binary_expr(&mut self, expr: &ExprBinary, ) -> CompileResultList<(ExprResolveRes, Vec<u8>)> {
        let (left_res, mut left_code) = expr.left.accept(self)?;
        let (right_res, mut right_code) = expr.right.accept(self)?;
        let expr_res = Self::pack_error(
            self.resolver
                .resolve_binary_expr(expr, &left_res, &right_res),
        )?;
        let expr_code = Self::pack_error(self.compiler.compile_binary_expr(
            expr,
            &expr_res,
            &mut left_code,
            &left_res,
            &mut right_code,
            &right_res,
        ))?;
        Ok((expr_res, expr_code))
    }

    fn visit_grouping_expr(&mut self, expr: &ExprGrouping) -> CompileResultList<(ExprResolveRes, Vec<u8>)> {
        let (inside_res, mut inside_code) = expr.expression.accept(self)?;
        let expr_res = Self::pack_error(self.resolver.resolve_grouping_expr(&inside_res))?;
        let expr_code = Self::pack_error(self.compiler.compile_grouping_expr(&mut inside_code))?;
        Ok((expr_res, expr_code))
    }

    fn visit_literal_expr(&mut self, expr: &ExprLiteral) -> CompileResultList<(ExprResolveRes, Vec<u8>)> {
        let expr_res = Self::pack_error(self.resolver.resolve_literal_expr(expr))?;
        let expr_code = Self::pack_error(self.compiler.compile_literal_expr(expr))?;
        Ok((expr_res, expr_code))
    }

    fn visit_unary_expr(&mut self, expr: &ExprUnary) -> CompileResultList<(ExprResolveRes, Vec<u8>)> {
        let (right_res, mut right_code) = expr.right.accept(self)?;
        let expr_res = Self::pack_error(self.resolver.resolve_unary_expr(expr, &right_res))?;
        let expr_code = Self::pack_error(self.compiler.compile_unary_expr(
            expr,
            &mut right_code,
            &right_res,
        ))?;
        Ok((expr_res, expr_code))
    }

    fn visit_as_expr(&mut self, expr: &ExprAs) -> CompileResultList<(ExprResolveRes, Vec<u8>)> {
        let (inside_res, mut inside_code) = expr.expression.accept(self)?;
        let expr_res = Self::pack_error(self.resolver.resolve_as_expr(expr, &inside_res))?;
        let expr_code =
            Self::pack_error(self.compiler.compile_as_expr(&expr_res, &mut inside_code))?;
        Ok((expr_res, expr_code))
    }

    fn visit_variable_expr(&mut self, expr: &ExprVariable) -> CompileResultList<(ExprResolveRes, Vec<u8>)> {
        let (expr_res, slot, is_ref) = Self::pack_error(self.resolver.resolve_variable_expr(expr))?;
        let expr_code = Self::pack_error(self.compiler.compile_variable_expr(
            &expr_res,
            slot,
            self.context.in_assign,
            self.context.in_ref_let,
            is_ref,
        ))?;
        Ok((expr_res, expr_code))
    }

    fn visit_call_expr(&mut self, expr: &ExprCall) -> CompileResultList<(ExprResolveRes, Vec<u8>)> {
        let (mut arg_res, mut arg_code) = (vec![], vec![]);
        for arg in &expr.arguments {
            let (res, code) = arg.accept(self)?;
            arg_res.push(res);
            arg_code.push(code);
        }
        
        // 查找函数
        let functions = if self.func_search_list.contains_key(&expr.func_name) {
            self.func_search_list.get(&expr.func_name).unwrap()
        } else {
            return Self::pack_error(Err(CompileError::new(&expr.pos, "Function not found.".to_string())));
        };
        
        // 对比参数类型，找到函数
        let mut target = None;
        for function in functions {
            let mut found = arg_res.len() == function.get_params().len();
            if found {
                for (res, (_, expect)) in arg_res.iter().zip(function.get_params().iter()) {
                    if &res.res_type != expect {
                        found = false;
                        break;
                    }
                }
            }
            if found {
                target = Some(function);
            }
        }
        
        if let None = target {
            return Self::pack_error(Err(CompileError::new(&expr.pos, "No overload version of this function is satisfied.".to_string())));
        }
        
        let func = target.unwrap();
        
        let final_code = Self::pack_error(self.compiler.compile_call_expr(&arg_res, &mut arg_code, func))?;
        
        let ope_type = func.get_return_type().clone();
        let res_type = func.get_return_type().clone();
        
        Ok((ExprResolveRes::new(res_type, ope_type), final_code))
    }
}
