//! 前端编译器模块

use crate::compiler::Compiler;
use crate::errors::error_types::{CompileError, CompileResultList};
use crate::instr::Instruction;
use crate::resolver::Resolver;
use crate::stmt::Stmt;

mod front_compiler_assistance;
mod front_compiler_expr;
mod front_compiler_stmt;

/// 前端编译器
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct FrontCompiler<'a> {
    pub resolver: Resolver,
    pub compiler: Compiler,
    statements: &'a [Stmt],
    context: Context,
    break_patches: Vec<BreakPatch>,
    continue_patches: Vec<ContinuePatch>,
    codes: Vec<u8>,
}

impl<'a> FrontCompiler<'a> {
    #[must_use]
    pub fn new(statements: &'a [Stmt]) -> Self {
        Self {
            resolver: Resolver::new(),
            compiler: Compiler::new(),
            statements,
            context: Context::init(),
            break_patches: vec![],
            continue_patches: vec![],
            codes: vec![],
        }
    }

    /// 启动编译
    pub fn compile(mut self) -> CompileResultList<Vec<u8>> {
        let mut errors = vec![];

        self.resolver.enter_scope();

        self.compile_scope(&mut errors, self.statements);

        self.resolver.leave_scope();

        // 补充返回指令，临时充当结束程序的作用
        self.write_code(Instruction::OpReturn);

        return if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(self.codes)
        };
    }

    /// 编译一个作用域
    pub fn compile_scope(&mut self, errors: &mut Vec<CompileError>, statements: &[Stmt]) {
        if let Err(mut errs) = self.resolver.predefine(statements) {
            errors.append(&mut errs);
        }

        for statement in statements {
            if let Err(mut err) = statement.accept(self) {
                errors.append(&mut err);
            }
        }
    }
}

/// 上下文
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Context {
    in_assign: bool,
    in_ref_let: bool,
    in_loop: bool,
    loop_tags: Vec<Option<String>>,
}

impl Context {
    pub fn init() -> Self {
        Self {
            in_assign: false,
            in_ref_let: false,
            in_loop: false,
            loop_tags: vec![],
        }
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
