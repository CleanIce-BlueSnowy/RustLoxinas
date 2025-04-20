//! 编译器——语句模块

use crate::compiler::Compiler;
use crate::data::DataSize;
use crate::errors::error_types::{CompileError, CompileResult};
use crate::instr::Instruction::*;
use crate::position::Position;
use crate::resolver::ExprResolveRes;
use crate::types::ValueType;

impl Compiler {
    /// 编译表达式语句
    pub fn compile_expr_stmt(
        &mut self,
        expr_code: &mut Vec<u8>,
        expr_res: &ExprResolveRes,
    ) -> CompileResult<Vec<u8>> {
        let mut target = vec![];
        target.append(expr_code);

        self.write_code(match expr_res.res_type.get_size() {
            // 弹出结果
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
    pub fn compile_let_stmt(
        &mut self,
        init_code: Option<&mut Vec<u8>>,
        init_res: Option<&ExprResolveRes>,
        target_type: ValueType,
        slot: usize,
        in_loop: bool,
    ) -> CompileResult<Vec<u8>> {
        let init_type = if let Some(res) = init_res {
            Some(&res.res_type)
        } else {
            None
        };

        // 当前栈顶就是变量的偏移位置
        let mut target = vec![];
        if let Some(code) = init_code {
            target.append(code);
            self.convert_types(init_type.as_ref().unwrap(), &target_type);

            // 循环已预分配内存，只需写入
            if in_loop {
                self.write_code(match target_type.get_size() {
                    DataSize::Byte => OpSetLocalByte,
                    DataSize::Word => OpSetLocalWord,
                    DataSize::Dword => OpSetLocalDword,
                    DataSize::Qword => OpSetLocalQword,
                    DataSize::Oword => OpSetLocalOword,
                });
                self.write_arg_dword((slot as u32).to_le_bytes());
            }

            self.append_temp_chunk(&mut target);
        } else {
            // 填充占位符（循环中已预分配内存，不需要填充）
            if !in_loop {
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
            }
            self.append_temp_chunk(&mut target);
        }

        return Ok(target);
    }

    /// 编译变量延迟初始化语句
    pub fn compile_init_stmt(
        &mut self,
        slot: usize,
        right_slot: Option<usize>,
        init_code: &mut Vec<u8>,
        init_res: &ExprResolveRes,
        target_type: ValueType,
    ) -> CompileResult<Vec<u8>> {
        let mut target = vec![];

        // 引用变量初始化需要写入左值的偏移地址
        if let Some(right_slot) = right_slot {
            self.write_code(OpPushDword);
            self.write_arg_dword((right_slot as u32).to_le_bytes());
        } else {
            target.append(init_code);
        }

        self.convert_types(&init_res.res_type, &target_type);

        self.write_code(match target_type.get_size() {
            DataSize::Byte => OpSetLocalByte,
            DataSize::Word => OpSetLocalWord,
            DataSize::Dword => OpSetLocalDword,
            DataSize::Qword => OpSetLocalQword,
            DataSize::Oword => OpSetLocalOword,
        });
        self.write_arg_dword((slot as u32).to_le_bytes());

        self.append_temp_chunk(&mut target);

        return Ok(target);
    }

    /// 编译赋值语句
    pub fn compile_assign_stmt(
        &mut self,
        vars_code: &mut [Vec<u8>],
        vars_res: &[ExprResolveRes],
        right_code: &mut Vec<u8>,
        right_res: &ExprResolveRes,
    ) -> CompileResult<Vec<u8>> {
        let mut target = vec![];

        // 写入赋值源
        target.append(right_code);

        // 除第一个，其他的赋值源需要复制
        for i in (1..vars_code.len()).rev() {
            let var_code = &mut vars_code[i];
            let var_res = &vars_res[i];
            self.write_code(match var_res.res_type.get_size() {
                DataSize::Byte => OpCopyByte,
                DataSize::Word => OpCopyWord,
                DataSize::Dword => OpCopyDword,
                DataSize::Qword => OpCopyQword,
                DataSize::Oword => OpCopyOword,
            });
            self.convert_types(&right_res.res_type, &var_res.res_type);
            self.append_temp_chunk(&mut target);
            target.append(var_code);
        }

        // 第一个直接赋值
        let var_code = &mut vars_code[0];
        let var_res = &vars_res[0];
        self.convert_types(&right_res.res_type, &var_res.res_type);
        self.append_temp_chunk(&mut target);
        target.append(var_code);

        return Ok(target);
    }

    /// 临时辅助功能：编译打印语句
    pub fn compile_print_stmt(
        &mut self,
        expr_code: Option<Vec<u8>>,
        expr_res: Option<ExprResolveRes>,
        expr_pos: Option<Position>,
    ) -> CompileResult<Vec<u8>> {
        use crate::instr::SpecialFunction::*;

        let mut target = vec![];

        self.write_code(OpSpecialFunction);
        if let Some(mut code) = expr_code {
            target.append(&mut code);
            self.write_special_func(match &expr_res.as_ref().unwrap().res_type {
                ValueType::Integer(integer) => {
                    use crate::types::ValueIntegerType::*;
                    match integer {
                        Byte => PrintByte,
                        SByte => PrintSByte,
                        Short => PrintShort,
                        UShort => PrintUShort,
                        Int => PrintInt,
                        UInt => PrintUInt,
                        Long => PrintLong,
                        ULong => PrintULong,
                        ExtInt => PrintExtInt,
                        UExtInt => PrintUExtInt,
                    }
                }
                ValueType::Float(float) => {
                    use crate::types::ValueFloatType::*;
                    match float {
                        Float => PrintFloat,
                        Double => PrintDouble,
                    }
                }
                ValueType::Bool => PrintBool,
                ValueType::Char => PrintChar,
                _ => {
                    return Err(CompileError::new(
                        &expr_pos.unwrap(),
                        format!(
                            "Cannot print the value of type '{}'.",
                            expr_res.unwrap().res_type
                        ),
                    ))
                }
            });
        } else {
            self.write_special_func(PrintNewLine);
        }

        self.append_temp_chunk(&mut target);

        return Ok(target);
    }
}
