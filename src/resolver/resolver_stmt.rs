//! 语义分析——语句分析模块

use crate::errors::error_types::{CompileError, CompileResult};
use crate::expr_get_pos;
use crate::resolver::{ExprResolveRes, Resolver};
use crate::stmt::{StmtInit, StmtLet};
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
            return Err(CompileError::new(&stmt.let_pos, "Must provide at least one of the type identifiers and initialization expressions.".to_string()));
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
    
    /// 分析变量延迟初始化语句
    pub fn resolve_init_stmt(&mut self, 
                             stmt: &StmtInit, 
                             init_expr_res: &ExprResolveRes) -> CompileResult<(ValueType, usize)> {
        let variable = if let Some(temp) = self.find_variable_in_current_scope(&stmt.name) {
            temp
        } else {
            return Err(CompileError::new(
                &stmt.name_pos,
                // 更加明确的错误信息
                if let Some(_) = self.find_variable(&stmt.name) {
                    "Can only be used at the same scope level as the variable definition.".to_string()
                } else {
                    "Undefined variable.".to_string()
                }
            ));
        };
        
        if !variable.defined {
            return Err(CompileError::new(&stmt.name_pos, "Initialize a variable before it's defined.".to_string()));
        }
        
        variable.initialized = true;
        // 检查类型转换
        if !Self::check_type_parse(&init_expr_res.res_type, &variable.var_type.as_ref().unwrap()) {
            return Err(CompileError::new(&expr_get_pos!(stmt.init.as_ref()), format!("Cannot use 'as' to convert '{}' to '{}'.", init_expr_res.res_type, variable.var_type.as_ref().unwrap())));
        }
        return Ok((variable.var_type.as_ref().unwrap().clone(), variable.slot));
    }
}
