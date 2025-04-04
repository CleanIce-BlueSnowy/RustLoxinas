//! 语义分析——语句分析模块

use crate::data::DataSize;
use crate::errors::error_types::{CompileError, CompileResult};
use crate::expr_get_pos;
use crate::resolver::{ExprResolveRes, Resolver, Variable};
use crate::stmt::{StmtAssign, StmtInit, StmtLet};
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
                if !Self::check_type_convert(&expr_res.res_type, ty) {
                    let pos = &stmt.var_type.as_ref().unwrap().pos;
                    return Err(CompileError::new(pos, format!("Cannot convert '{}' to '{}'", expr_res.res_type, ty)));
                }
            } else {
                variable.var_type = Some(expr_res.res_type.clone());
            }
        }
        
        // 处理无法推断类型的情况
        if let (None, None) = (&variable.var_type, init_expr_res) {
            return Err(CompileError::new(&stmt.let_pos, "Must provide at least one of the type identifier and initialization expression.".to_string()));
        }

        // 填写栈偏移量
        variable.slot = self.now_slot;
        let temp_size = &variable.var_type.as_ref().unwrap().get_size();
        self.update_slot(if stmt.is_ref {
            &DataSize::Word
        } else {
            temp_size
        });
        
        // 重写变量
        let target_variable = self.find_variable(&stmt.name).unwrap();
        target_variable.defined = variable.defined;
        target_variable.initialized = variable.initialized;
        target_variable.slot = variable.slot;
        target_variable.var_type = variable.var_type;
        
        // 如果是引用，判断左值表达式
        if stmt.is_ref {
            if let Some(expr) = &stmt.init {
                if !Self::check_left_value(expr) {
                    return Err(CompileError::new(&expr_get_pos!(expr), "Expect a left value expression.".to_string()));
                }
            } else {
                return Err(CompileError::new(&stmt.name_pos, "Must provide an initialization expression.".to_string()));
            }
        }
        
        return Ok(target_variable.var_type.clone().unwrap());
    }
    
    /// 分析变量延迟初始化语句
    pub fn resolve_init_stmt(&mut self, 
                             stmt: &StmtInit, 
                             init_expr_res: &ExprResolveRes) -> CompileResult<(ValueType, usize)> {
        let variable = if let Some(temp) = self.find_variable(&stmt.name) {
            temp
        } else {
            return Err(CompileError::new(
                &stmt.name_pos,
                "Undefined variable.".to_string(),
            ));
        };
        
        if !variable.defined {
            return Err(CompileError::new(&stmt.name_pos, "Initialize a variable before it's defined.".to_string()));
        }
        
        if variable.initialized {
            return Err(CompileError::new(&stmt.name_pos, "Variable has already been initialized.".to_string()));
        }
        
        variable.initialized = true;
        let ptr = variable as *mut Variable;
        
        // 上一个 variable 的作用域在此截止
        
        self.get_current_scope().init_vars.push(ptr);
        
        // 避免多次 self 可变引用
        let variable = self.find_variable(&stmt.name).unwrap();
        
        // 检查类型转换
        if !Self::check_type_convert(&init_expr_res.res_type, &variable.var_type.as_ref().unwrap()) {
            return Err(CompileError::new(&expr_get_pos!(&stmt.init), format!("Cannot use 'as' to convert '{}' to '{}'.", init_expr_res.res_type, variable.var_type.as_ref().unwrap())));
        }
        return Ok((variable.var_type.as_ref().unwrap().clone(), variable.slot));
    }
    
    /// 分析变量赋值语句
    pub fn resolve_assign_stmt(&mut self,
                               stmt: &StmtAssign, 
                               vars_res: &[ExprResolveRes], 
                               right_res: &ExprResolveRes) -> CompileResult<()> {
        // 检查左值和类型转换
        for idx in 0..vars_res.len() {
            let var = &stmt.assign_vars[idx];
            let var_res = &vars_res[idx];
            if !Self::check_left_value(var) {
                return Err(CompileError::new(&expr_get_pos!(var), "Except a left value expression.".to_string()));
            }
            if !Self::check_type_convert(&right_res.res_type, &var_res.res_type) {
                return Err(CompileError::new(&expr_get_pos!(var), format!("Cannot convert {} to {}", right_res.res_type, var_res.res_type)));
            }
        }
        
        return Ok(());
    }
    
    /// 临时辅助功能：分析打印语句
    pub fn resolve_print_stmt(&mut self) -> CompileResult<()> {
        Ok(())
    }
}
