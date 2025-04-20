//! 前端编译器辅助模块

use crate::byte_handler::byte_writer::{
    write_byte, write_dword, write_oword, write_qword, write_word,
};
use crate::errors::error_types::{CompileError, CompileResult};
use crate::front_compiler::FrontCompiler;
use crate::instr::{Instruction, SpecialFunction};
use crate::position::Position;
use crate::resolver::Scope;

impl<'a> FrontCompiler<'a> {
    /// 打包错误到错误列表
    #[inline]
    pub fn pack_error<OkType, ErrType>(
        result: Result<OkType, ErrType>,
    ) -> Result<OkType, Vec<ErrType>> {
        match result {
            Ok(ok) => Ok(ok),
            Err(err) => Err(vec![err]),
        }
    }

    /// 添加新指令
    #[inline]
    pub fn write_code(&mut self, instr: Instruction) {
        write_byte(&mut self.codes, [instr.into()]);
    }

    /// 写入特殊功能
    #[inline]
    pub fn write_special_func(&mut self, special_func: SpecialFunction) {
        write_byte(&mut self.codes, [special_func.into()]);
    }

    /// 添加字节参数
    #[inline]
    pub fn write_arg_byte(&mut self, byte: [u8; 1]) {
        write_byte(&mut self.codes, byte);
    }

    /// 添加单字参数
    #[inline]
    pub fn write_arg_word(&mut self, word: [u8; 2]) {
        write_word(&mut self.codes, word);
    }

    /// 添加双字参数
    #[inline]
    pub fn write_arg_dword(&mut self, dword: [u8; 4]) {
        write_dword(&mut self.codes, dword);
    }

    /// 添加四字参数
    #[inline]
    pub fn write_arg_qword(&mut self, qword: [u8; 8]) {
        write_qword(&mut self.codes, qword);
    }

    /// 添加八字参数
    #[inline]
    pub fn write_arg_oword(&mut self, oword: [u8; 16]) {
        write_oword(&mut self.codes, oword);
    }

    /// 检查作用域初始变量是否相同
    #[inline]
    pub fn scopes_same_inits(scope1: &Scope, scope2: &Scope) -> bool {
        scope1.init_vars == scope2.init_vars
    }

    /// 初始化作用域中的变量
    #[inline]
    pub fn scope_init_vars(scope: &Scope) {
        for &var in &scope.init_vars {
            unsafe {
                (*var).initialized = true;
            }
        }
    }

    /// 检查标记
    pub fn check_tag(&self, tag: &Option<String>, pos: &Position) -> CompileResult<()> {
        if let Some(tag_name) = tag {
            let mut found_tag = false;
            for tag in &self.context.loop_tags {
                if let Some(loop_tag) = tag {
                    if loop_tag == tag_name {
                        found_tag = true;
                        break;
                    }
                }
            }
            if !found_tag {
                Err(CompileError::new(
                    pos,
                    format!("Undefined tag: @{}", tag_name),
                ))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}
