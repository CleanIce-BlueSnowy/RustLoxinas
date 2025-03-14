use std::collections::HashMap;
use crate::data::DataSize;
use crate::errors::error_types::{CompileError, CompileResult};
use crate::hashmap;
use crate::resolver::{Resolver, Variable};
use crate::stmt::Stmt;
use crate::types::{TypeTag, ValueType};
use crate::types::ValueType::Object;

impl Resolver {
    /// 预定义
    pub fn predefine(&mut self, statements: &[Stmt]) -> Result<(), Vec<CompileError>> {
        let current = self.get_current_scope();
        let mut errors = Vec::new();
        for statement in statements {
            match statement {
                Stmt::Let(stmt) => {
                    if current.0.contains_key(&stmt.name) {
                        errors.push(CompileError::new(&stmt.name_pos, format!("Redefine variable '{}'.", &stmt.name)));
                    }
                    current.0.insert(stmt.name.clone(), Variable::new(
                        statement,
                        false,
                        false,
                        0,
                        None,
                    ));
                }
                _ => (),
            }
        }
        return if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        };
    }
    
    /// 解析类型标识符
    pub fn parse_value_type(&mut self, type_tag: &TypeTag) -> CompileResult<ValueType> {
        let mut res_type: Option<ValueType> = None;
        let mut search_map = self.global_types.clone();
        let mut in_global = true;
        for name in &type_tag.chain {
            if let Some(temp_ty) = &res_type {
                if let Object(object) = temp_ty {
                    search_map = object.get_contain_types().clone();
                    in_global = false;
                } else {
                    return Err(CompileError::new(&type_tag.pos, format!("Unknown type '{}' in '{}'.", name, temp_ty)));
                }
            }
            let ty = if let Some(temp) = search_map.get(name) {
                temp
            } else {
                return Err(CompileError::new(&type_tag.pos,
                                             if in_global {
                                                 format!("Unknown type '{}' in global.", name)
                                             } else {
                                                 format!("Unknown type '{}' in '{}'.", name, res_type.as_ref().unwrap())
                                             }));
            };
            res_type = Some(ty.clone());
        }

        // 不允许转换为对象
        if let Some(Object(_)) = res_type {
            return Err(CompileError::new(&type_tag.pos, "Cannot convert a value to an object by using 'as'.".to_string()));
        }
        
        return Ok(res_type.unwrap());
    }
    
    /// 进入作用域
    #[inline]
    pub fn enter_scope(&mut self) {
        self.variables.push((hashmap!(), self.now_slot));
    }
    
    /// 离开作用域
    #[inline]
    pub fn leave_scope(&mut self) {
        let (_, slot) = self.variables.pop().unwrap();
        self.now_slot = slot;
    }

    /// 获取当前作用域
    #[inline]
    pub fn get_current_scope(&mut self) -> &mut (HashMap<String, Variable>, usize) {
        let length = self.variables.len();
        return &mut self.variables[length - 1];
    }

    /// 寻找变量
    pub fn find_variable(&mut self, name: &String) -> Option<&mut Variable> {
        let mut scope_idx = self.variables.len();
        while scope_idx > 0 {
            let scope = &mut self.variables[scope_idx - 1].0;
            if scope.contains_key(name) {
                // 避免中间变量导致的循环生命周期异常，从而导致多次可变借用
                // 见：https://zhuanlan.zhihu.com/p/449797793
                return Some(self.variables[scope_idx - 1].0.get_mut(name).unwrap());
            }
            scope_idx -= 1;
        }
        return None;
    }
    
    /// 在当前作用域寻找变量
    pub fn find_variable_in_current_scope(&mut self, name: &String) -> Option<&mut Variable> {
        let current = self.get_current_scope();
        return current.0.get_mut(name);
    }
    
    /// 检查类型转换
    #[inline]
    pub fn check_type_parse(from: &ValueType, to: &ValueType) -> bool {
        if let (Object(_), _) | (_, Object(_)) = (from, to) {
            return false;
        }
        return true;
    }
    
    /// 更新当前栈偏移量
    #[inline]
    pub fn update_slot(&mut self, data_size: &DataSize) {
        self.now_slot += data_size.get_bytes_cnt();
    }
}
