use crate::data::DataSize;
use crate::errors::error_types::CompileError;
use crate::expr::Expr;
use crate::resolver::{Resolver, Scope, Variable};
use crate::stmt::Stmt;
use crate::types::ValueType::Object;
use crate::types::ValueType;

impl Resolver {
    /// 预定义
    pub fn predefine(&mut self, statements: &[Stmt]) -> Result<(), Vec<CompileError>> {
        let current = self.get_current_scope();
        let mut errors = Vec::new();

        for statement in statements {
            match statement {
                Stmt::Let(stmt) => {
                    if current.variables.contains_key(&stmt.name) {
                        errors.push(CompileError::new(
                            &stmt.name_pos,
                            format!("Redefine variable '{}'.", &stmt.name),
                        ));
                    }
                    current.add_variable(
                        stmt.name.clone(),
                        Variable::new(false, false, 0, None, stmt.is_ref),
                    );
                }
                _ => (),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 进入作用域
    #[inline]
    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::new(self.now_slot));
    }

    /// 离开作用域
    #[inline]
    pub fn leave_scope(&mut self) -> Scope {
        let scope = self.scopes.pop().unwrap();

        // 重置初始化变量
        // 当某个语句有多个子语句块时，分析器需要检查每个子语句块的初始化变量是否一致，然后选择性地确认初始化
        for &variable in &scope.init_vars {
            // SAFETY: Scope 的 init_vars 一定引用的是上一层作用域的变量，所以安全
            unsafe {
                (*variable).initialized = false;
            }
        }

        self.now_slot = scope.top_slot;

        scope
    }

    /// 获取当前作用域
    #[inline]
    #[must_use]
    pub fn get_current_scope(&mut self) -> &mut Scope {
        let length = self.scopes.len();
        &mut self.scopes[length - 1]
    }

    /// 寻找变量
    #[must_use]
    pub fn find_variable(&mut self, name: &String) -> Option<&mut Variable> {
        let mut scope_idx = self.scopes.len();
        while scope_idx > 0 {
            let var_scope = &mut self.scopes[scope_idx - 1].variables;
            if var_scope.contains_key(name) {
                // 避免中间变量导致的循环生命周期异常，从而导致多次可变借用
                // 见：https://zhuanlan.zhihu.com/p/449797793
                return Some(self.scopes[scope_idx - 1].variables.get_mut(name).unwrap());
            }
            scope_idx -= 1;
        }
        None
    }

    /// 在当前作用域寻找变量
    #[inline]
    #[must_use]
    pub fn find_variable_in_current_scope(&mut self, name: &String) -> Option<&mut Variable> {
        let current = self.get_current_scope();
        current.variables.get_mut(name)
    }

    /// 检查类型转换
    #[must_use]
    pub fn check_type_convert(from: &ValueType, to: &ValueType) -> bool {
        if let (Object(_), _) | (_, Object(_)) = (from, to) {
            return false;
        }
        true
    }

    /// 更新当前栈偏移量
    #[inline]
    pub fn update_slot(&mut self, data_size: &DataSize) {
        self.now_slot += data_size.get_bytes_cnt();
    }

    /// 检查左值
    #[inline]
    #[must_use]
    pub fn check_left_value(expr: &Expr) -> bool {
        matches!(expr, Expr::Variable(_))
    }
}
