//! 编译器——语句模块

use std::collections::LinkedList;

use crate::compiler::Compiler;
use crate::data::DataSize;
use crate::errors::error_types::CompileResult;
use crate::instr::Instruction::*;
use crate::resolver::ExprResolveRes;
use crate::types::ValueType;

impl Compiler {
    /// 编译表达式语句
    pub fn compile_expr_stmt(&mut self,
                             expr_res: &ExprResolveRes,
                             expr_code: &mut LinkedList<u8>) -> CompileResult<LinkedList<u8>> {
        let mut target = LinkedList::new();
        target.append(expr_code);

        self.write_code(match expr_res.res_type.get_size() {  // 弹出结果
            DataSize::Byte => OpPopByte,
            DataSize::Word => OpPopWord,
            DataSize::Dword => OpPopDword,
            DataSize::Qword => OpPopQword,
            DataSize::Oword => OpPopOword,
        });
        self.append_temp_chunk(&mut target);
        
        return Ok(target);
    }

    /// 编译变量定义语句
    pub fn compile_let_stmt(&mut self,
                            init_code: Option<&mut LinkedList<u8>>,
                            init_res: Option<&ExprResolveRes>,
                            target_type: ValueType) -> CompileResult<LinkedList<u8>> {
        let init_type = if let Some(res) = init_res {
            Some(&res.res_type)
        } else {
            None
        };
        
        // 当前栈顶就是变量的偏移位置
        let mut target = LinkedList::new();
        if let Some(code) = init_code {
            target.append(code);
            self.convert_types(init_type.as_ref().unwrap(), &target_type);
            self.append_temp_chunk(&mut target);
        } else {
            // 填充占位符
            match target_type.get_size() {
                DataSize::Byte => {
                    self.write_code(OpPushByte);
                    self.write_arg_byte(0u8.to_le_bytes());
                }
                DataSize::Word => {
                    self.write_code(OpPushWord);
                    self.write_arg_word(0u16.to_le_bytes());
                }
                DataSize::Dword => {
                    self.write_code(OpPushDword);
                    self.write_arg_dword(0u32.to_le_bytes());
                }
                DataSize::Qword => {
                    self.write_code(OpPushQword);
                    self.write_arg_qword(0u64.to_le_bytes());
                }
                DataSize::Oword => {
                    self.write_code(OpPushOword);
                    self.write_arg_oword(0u128.to_le_bytes());
                }
            }
            self.append_temp_chunk(&mut target);
        }
        
        return Ok(target);
    }
    
    /// 编译变量延迟初始化语句
    pub fn compile_init_stmt(&mut self, 
                             slot: usize,
                             init_code: &mut LinkedList<u8>, 
                             init_res: &ExprResolveRes, 
                             target_type: ValueType) -> CompileResult<LinkedList<u8>> {
        let mut target = LinkedList::new();
        target.append(init_code);
        
        self.convert_types(&init_res.res_type, &target_type);
        
        self.write_code(
            match target_type.get_size() {
                DataSize::Byte => OpSetLocalByte,
                DataSize::Word => OpSetLocalWord,
                DataSize::Dword => OpSetLocalDword,
                DataSize::Qword => OpSetLocalQword,
                DataSize::Oword => OpSetLocalOword,
            }
        );
        self.write_arg_word((slot as u16).to_le_bytes());
        
        self.append_temp_chunk(&mut target);
        
        return Ok(target);
    }
}
