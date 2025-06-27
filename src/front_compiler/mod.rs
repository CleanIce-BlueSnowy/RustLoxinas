//! 前端编译器模块

use crate::compiler::Compiler;
use crate::errors::error_types::{CompileError, CompileResultList};
use crate::function::LoxinasFunction;
use crate::instr::Instruction;
use crate::resolver::Resolver;
use crate::stmt::Stmt;
use std::collections::HashMap;
use crate::position::Position;
use crate::stmt_get_pos;
use crate::types::ValueType;

mod front_compiler_assistance;
mod front_compiler_expr;
mod front_compiler_stmt;

/// 前端编译器
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct FrontCompiler<'a> {
    /// 语义分析
    pub resolver: Resolver,
    /// 简单编译
    pub compiler: Compiler,
    /// 当前函数
    current_function: &'a LoxinasFunction,
    /// 函数名称搜索
    func_search_list: &'a HashMap<String, Vec<LoxinasFunction>>,
    /// 语句
    statements: &'a [Stmt],
    /// 上下文
    context: Context,
    /// 循环标记
    loop_tags: Vec<Option<String>>,
    /// `break` 补丁
    break_patches: Vec<BreakPatch>,
    /// `continue` 补丁
    continue_patches: Vec<ContinuePatch>,
    /// 生成的代码
    codes: Vec<u8>,
}

impl<'a> FrontCompiler<'a> {
    #[must_use]
    pub fn new(
        statements: &'a [Stmt],
        function: &'a LoxinasFunction,
        func_search_list: &'a HashMap<String, Vec<LoxinasFunction>>,
    ) -> Self {
        Self {
            resolver: Resolver::new(),
            compiler: Compiler::new(),
            current_function: function,
            func_search_list,
            statements,
            context: Context::init(),
            loop_tags: vec![],
            break_patches: vec![],
            continue_patches: vec![],
            codes: vec![],
        }
    }

    /// 启动编译
    pub fn compile(mut self) -> CompileResultList<Vec<u8>> {
        let mut errors = vec![];

        self.resolver.enter_scope();

        self.resolver.init_parameters(self.current_function.get_params());
        self.compile_scope(&mut errors, self.statements);

        self.resolver.leave_scope();

        if !self.context.returned {
            // 补充返回指令
            if self.current_function.get_return_type() != &ValueType::Unit {
                let final_stmt = &self.statements[self.statements.len() - 1];
                let final_pos = stmt_get_pos!(final_stmt);
                errors.push(CompileError::new(
                    &Position::new(
                        final_pos.end_line,
                        final_pos.end_idx,
                        final_pos.end_line,
                        final_pos.end_idx + 1,
                    ),
                    "Cannot return default value `unit`: type mismatched. Expect explicit return.".to_string(),
                ));
            }
            self.write_code(Instruction::OpReturnUnit);
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(self.codes)
        }
    }

    /// 编译一个作用域
    pub fn compile_scope(&mut self, errors: &mut Vec<CompileError>, statements: &[Stmt]) {
        if let Err(mut errs) = self.resolver.predefine(statements) {
            errors.append(&mut errs);
        }

        self.context.final_statement = false;
        for (i, statement) in statements.iter().enumerate() {
            if i == statements.len() - 1 {
                self.context.final_statement = true;
            }
            if let Err(mut err) = statement.accept(self) {
                errors.append(&mut err);
            }
        }
    }
}

/// 上下文
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct Context {
    in_assign: bool,
    in_ref_let: bool,
    in_loop: bool,
    final_statement: bool,
    returned: bool,
    in_if_branch: bool,
}

impl Context {
    pub fn init() -> Self {
        Self {
            in_assign: false,
            in_ref_let: false,
            in_loop: false,
            final_statement: false,
            returned: false,
            in_if_branch: false,
        }
    }
    
    /// 保存上下文
    pub fn save(&self) -> Self {
        self.clone()
    }
    
    /// 恢复上下文
    pub fn restore(&mut self, source: Self) {
        self.in_assign = source.in_assign;
        self.in_ref_let = source.in_ref_let;
        self.in_loop = source.in_loop;
        self.final_statement = source.final_statement;
        self.returned = source.returned;
        self.in_if_branch = source.in_if_branch;
    }
}

/// 跳出循环补丁
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct BreakPatch {
    loop_tag: Option<String>,
    patch_pos: u32,
}

impl BreakPatch {
    pub fn new(loop_tag: Option<String>, break_pos: u32) -> Self {
        Self {
            loop_tag,
            patch_pos: break_pos,
        }
    }
}

/// 继续循环补丁
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ContinuePatch {
    loop_tag: Option<String>,
    patch_pos: u32,
}

impl ContinuePatch {
    pub fn new(loop_tag: Option<String>, continue_pos: u32) -> Self {
        Self {
            loop_tag,
            patch_pos: continue_pos,
        }
    }
}
