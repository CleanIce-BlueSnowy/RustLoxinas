//! 语义分析——语句分析模块

use crate::errors::error_types::{CompileError, CompileResult};
use crate::resolver::{ExprResolveRes, Resolver};
use crate::stmt::StmtLet;
use crate::types::ValueType;

impl Resolver {
    /// 分析表达式语句
    pub fn resolve_expr_stmt(&mut self) -> CompileResult<()> {
        return Ok(());
    }

    /// 分析变量定义语句
    pub fn resolve_let_stmt(&mut self,
                            stmt: &StmtLet,
                            init_expr_res: Option<&ExprResolveRes>) -> CompileResult<ValueType> {
        // 定义变量，赋值以避免多次可变引用
        let mut variable = self.find_variable(&stmt.name).unwrap().clone();
        variable.defined = true;

        // 处理类型定义
        if let Some(tag) = &stmt.var_type {
            let ty = self.parse_value_type(tag)?;
            variable.var_type = Some(ty);
        }

        // 处理初始化表达式
        if let Some(expr_res) = init_expr_res {
            variable.initialized = true;
            if let Some(ty) = &variable.var_type {
                if !Self::check_type_parse(&expr_res.res_type, ty) {
                    let pos = &stmt.var_type.as_ref().unwrap().pos;
                    return Err(CompileError::new(pos, format!("Cannot convert '{}' to '{}'", expr_res.res_type, ty)));
                }
            } else {
                variable.var_type = Some(expr_res.res_type.clone());
            }
        }
        
        // 处理无法推断类型的情况
        if let (None, None) = (&variable.var_type, init_expr_res) {
            return Err(CompileError::new(&stmt.let_pos, "Cannot know the type.".to_string()));
        }

        // 填写栈偏移量
        variable.slot = self.now_slot;
        self.update_slot(&variable.var_type.as_ref().unwrap().get_size());
        
        // 重写变量
        let target_variable = self.find_variable(&stmt.name).unwrap();
        target_variable.defined = variable.defined;
        target_variable.initialized = variable.initialized;
        target_variable.slot = variable.slot;
        target_variable.var_type = variable.var_type;
        
        return Ok(target_variable.var_type.clone().unwrap());
    }
}
