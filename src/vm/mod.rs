//! 虚拟机模块

use crate::byte_handler::byte_reader::read_byte;

#[cfg(debug_assertions)]
use crate::disassembler::disassemble_instruction;

use crate::errors::error_types::{RuntimeError, RuntimeResult};
use crate::instr::{Instruction, SpecialFunction};
use crate::instr::Instruction::*;

mod vm_assistance;
mod vm_debug;
mod vim_io;

pub struct VM<'a> {
    pub vm_stack: Vec<u8>,
    pub chunk: &'a [u8],
    pub ip: usize,
    pub frame_start: usize,
}

impl<'a> VM<'a> {
    #[must_use]
    pub fn new(chunk: &'a [u8]) -> Self {
        Self { vm_stack: vec![], chunk, ip: 0, frame_start: 0 }
    }
    
    /// 运行字节码
    pub fn run(&mut self) -> RuntimeResult<()> {
        while self.ip < self.chunk.len() {
            #[cfg(debug_assertions)]
            let old_ip = self.ip;

            let instr_byte;
            match read_byte(self.chunk, self.ip) {
                Ok(temp) => instr_byte = temp.0[0],
                Err(_) => panic!("Unexpected error!"),
            }

            self.ip += 1;

            let instr = if let Ok(temp) = Instruction::try_from(instr_byte) {
                temp
            } else {
                return Err(RuntimeError::new(format!("Unknown instruction: {:02x}", instr_byte)));
            };

            #[cfg(debug_assertions)]
            {
                self.print_stack();
                match disassemble_instruction(instr.clone(), self.chunk, old_ip + 1) {
                    Ok(temp) => println!("{:08X} {}", old_ip, temp.0),
                    Err(err) => return Err(RuntimeError::new(format!("Disassembler threw an error: {}", err))),
                }
            }

            self.run_code(instr)?;
        }
        
        #[cfg(debug_assertions)]
        {
            print!("FINALLY ");
            self.print_stack();
        }

        return Ok(());
    }
    
    /// 运行单条指令
    #[inline]
    fn run_code(&mut self, instr: Instruction) -> RuntimeResult<()> {
        match instr {
            OpSpecialFunction => {
                let func_byte;
                match read_byte(self.chunk, self.ip) {
                    Ok(temp) => func_byte = temp.0[0],
                    Err(_) => panic!("Unexpected error!"),
                }

                self.ip += 1;

                let special_func = if let Ok(temp) = SpecialFunction::try_from(func_byte) {
                    temp
                } else {
                    return Err(RuntimeError::new(format!("Unknown instruction: {:02x}", func_byte)));
                };
                
                self.run_special_function(special_func)?;
            }
            OpReturn => {  // 临时充当结束程序的作用
                return Ok(());
            }
            OpStackExtend => {
                let length = u32::from_le_bytes(self.read_arg_dword());
                self.stack_extend(length);
            }
            OpStackShrink => {
                let length = u32::from_le_bytes(self.read_arg_dword());
                self.stack_shrink(length);
            }
            OpJump => {
                let goto = i32::from_le_bytes(self.read_arg_dword());
                self.jump(goto);
            }
            OpJumpTrue => {
                let goto = i32::from_le_bytes(self.read_arg_dword());
                let condition = self.peek_bool();
                if condition {
                    self.jump(goto);
                }
            }
            OpJumpTruePop => {
                let goto = i32::from_le_bytes(self.read_arg_dword());
                let condition = self.pop_bool();
                if condition {
                    self.jump(goto);
                }
            }
            OpJumpFalse => {
                let goto = i32::from_le_bytes(self.read_arg_dword());
                let condition = self.peek_bool();
                if !condition {
                    self.jump(goto);
                }
            }
            OpJumpFalsePop => {
                let goto = i32::from_le_bytes(self.read_arg_dword());
                let condition = self.pop_bool();
                if !condition {
                    self.jump(goto);
                }
            }
            OpLoadConstByte => {
                let byte = self.read_arg_byte();
                self.push_byte(byte);
            }
            OpLoadConstWord => {
                let word = self.read_arg_word();
                self.push_word(word);
            }
            OpLoadConstDword => {
                let dword = self.read_arg_dword();
                self.push_dword(dword);
            }
            OpLoadConstQword => {
                let qword = self.read_arg_qword();
                self.push_qword(qword);
            }
            OpLoadConstOword => {
                let oword = self.read_arg_oword();
                self.push_oword(oword);
            }
            OpSignExtendByteToWord => {
                let high_byte = self.peek_byte()[0];
                if high_byte & 0b10000000 == 0 {
                    self.push_byte([0x00]);
                } else {
                    self.push_byte([0xff]);
                }
            }
            OpSignExtendWordToDword => {
                let high_byte = self.peek_byte()[0];
                if high_byte & 0b10000000 == 0 {
                    self.push_word([0x00, 0x00]);
                } else {
                    self.push_word([0xff, 0xff]);
                }
            }
            OpSignExtendDwordToQword => {
                let high_byte = self.peek_byte()[0];
                if high_byte & 0b10000000 == 0 {
                    self.push_dword([0x00, 0x00, 0x00, 0x00]);
                } else {
                    self.push_dword([0xff, 0xff, 0xff, 0xff]);
                }
            }
            OpSignExtendQwordToOword => {
                let high_byte = self.peek_byte()[0];
                if high_byte & 0b10000000 == 0 {
                    self.push_qword([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
                } else {
                    self.push_qword([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
                }
            }
            OpZeroExtendByteToWord => {
                self.push_byte([0x00]);
            }
            OpZeroExtendWordToDword => {
                self.push_word([0x00, 0x00]);
            }
            OpZeroExtendDwordToQword => {
                self.push_dword([0x00, 0x00, 0x00, 0x00]);
            }
            OpZeroExtendQwordToOword => {
                self.push_qword([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
            }
            OpTruncateOwordToQword => {
                self.pop_qword();
            }
            OpTruncateQwordToDword => {
                self.pop_dword();
            }
            OpTruncateDwordToWord => {
                self.pop_word();
            }
            OpTruncateWordToByte => {
                self.pop_byte();
            }
            OpIAddByte => {
                let num2 = u8::from_le_bytes(self.pop_byte());
                let num1 = u8::from_le_bytes(self.pop_byte());
                let res = num1.wrapping_add(num2);
                self.push_byte(res.to_le_bytes());
            }
            OpIAddWord => {
                let num2 = u16::from_le_bytes(self.pop_word());
                let num1 = u16::from_le_bytes(self.pop_word());
                let res = num1.wrapping_add(num2);
                self.push_word(res.to_le_bytes());
            }
            OpIAddDword => {
                let num2 = u32::from_le_bytes(self.pop_dword());
                let num1 = u32::from_le_bytes(self.pop_dword());
                let res = num1.wrapping_add(num2);
                self.push_dword(res.to_le_bytes());
            }
            OpIAddQword => {
                let num2 = u64::from_le_bytes(self.pop_qword());
                let num1 = u64::from_le_bytes(self.pop_qword());
                let res = num1.wrapping_add(num2);
                self.push_qword(res.to_le_bytes());
            }
            OpIAddOword => {
                let num2 = u128::from_le_bytes(self.pop_oword());
                let num1 = u128::from_le_bytes(self.pop_oword());
                let res = num1.wrapping_add(num2);
                self.push_oword(res.to_le_bytes());
            }
            OpISubByte => {
                let num2 = u8::from_le_bytes(self.pop_byte());
                let num1 = u8::from_le_bytes(self.pop_byte());
                let res = num1.wrapping_sub(num2);
                self.push_byte(res.to_le_bytes());
            }
            OpISubWord => {
                let num2 = u16::from_le_bytes(self.pop_word());
                let num1 = u16::from_le_bytes(self.pop_word());
                let res = num1.wrapping_sub(num2);
                self.push_word(res.to_le_bytes());
            }
            OpISubDword => {
                let num2 = u32::from_le_bytes(self.pop_dword());
                let num1 = u32::from_le_bytes(self.pop_dword());
                let res = num1.wrapping_sub(num2);
                self.push_dword(res.to_le_bytes());
            }
            OpISubQword => {
                let num2 = u64::from_le_bytes(self.pop_qword());
                let num1 = u64::from_le_bytes(self.pop_qword());
                let res = num1.wrapping_sub(num2);
                self.push_qword(res.to_le_bytes());
            }
            OpISubOword => {
                let num2 = u128::from_le_bytes(self.pop_oword());
                let num1 = u128::from_le_bytes(self.pop_oword());
                let res = num1.wrapping_sub(num2);
                self.push_oword(res.to_le_bytes());
            }
            OpIMulByte => {
                let num2 = u8::from_le_bytes(self.pop_byte());
                let num1 = u8::from_le_bytes(self.pop_byte());
                let res = num1.wrapping_mul(num2);
                self.push_byte(res.to_le_bytes());
            }
            OpIMulWord => {
                let num2 = u16::from_le_bytes(self.pop_word());
                let num1 = u16::from_le_bytes(self.pop_word());
                let res = num1.wrapping_mul(num2);
                self.push_word(res.to_le_bytes());
            }
            OpIMulDword => {
                let num2 = u32::from_le_bytes(self.pop_dword());
                let num1 = u32::from_le_bytes(self.pop_dword());
                let res = num1.wrapping_mul(num2);
                self.push_dword(res.to_le_bytes());
            }
            OpIMulQword => {
                let num2 = u64::from_le_bytes(self.pop_qword());
                let num1 = u64::from_le_bytes(self.pop_qword());
                let res = num1.wrapping_mul(num2);
                self.push_qword(res.to_le_bytes());
            }
            OpIMulOword => {
                let num2 = u128::from_le_bytes(self.pop_oword());
                let num1 = u128::from_le_bytes(self.pop_oword());
                let res = num1.wrapping_mul(num2);
                self.push_oword(res.to_le_bytes());
            }
            OpIDivSByte => {
                let num2 = i8::from_le_bytes(self.pop_byte());
                let num1 = i8::from_le_bytes(self.pop_byte());
                if num2 == 0 {
                    return Err(RuntimeError::new("Division by zero.".to_string()));
                }
                let res = num1.wrapping_div(num2);
                self.push_byte(res.to_le_bytes());
            }
            OpIDivSWord => {
                let num2 = i16::from_le_bytes(self.pop_word());
                let num1 = i16::from_le_bytes(self.pop_word());
                if num2 == 0 {
                    return Err(RuntimeError::new("Division by zero.".to_string()));
                }
                let res = num1.wrapping_div(num2);
                self.push_word(res.to_le_bytes());
            }
            OpIDivSDword => {
                let num2 = i32::from_le_bytes(self.pop_dword());
                let num1 = i32::from_le_bytes(self.pop_dword());
                if num2 == 0 {
                    return Err(RuntimeError::new("Division by zero.".to_string()));
                }
                let res = num1.wrapping_div(num2);
                self.push_dword(res.to_le_bytes());
            }
            OpIDivSQword => {
                let num2 = i64::from_le_bytes(self.pop_qword());
                let num1 = i64::from_le_bytes(self.pop_qword());
                if num2 == 0 {
                    return Err(RuntimeError::new("Division by zero.".to_string()));
                }
                let res = num1.wrapping_div(num2);
                self.push_qword(res.to_le_bytes());
            }
            OpIDivSOword => {
                let num2 = i128::from_le_bytes(self.pop_oword());
                let num1 = i128::from_le_bytes(self.pop_oword());
                if num2 == 0 {
                    return Err(RuntimeError::new("Division by zero.".to_string()));
                }
                let res = num1.wrapping_div(num2);
                self.push_oword(res.to_le_bytes());
            }
            OpIDivUByte => {
                let num2 = u8::from_le_bytes(self.pop_byte());
                let num1 = u8::from_le_bytes(self.pop_byte());
                if num2 == 0 {
                    return Err(RuntimeError::new("Division by zero.".to_string()));
                }
                let res = num1.wrapping_div(num2);
                self.push_byte(res.to_le_bytes());
            }
            OpIDivUWord => {
                let num2 = u16::from_le_bytes(self.pop_word());
                let num1 = u16::from_le_bytes(self.pop_word());
                if num2 == 0 {
                    return Err(RuntimeError::new("Division by zero.".to_string()));
                }
                let res = num1.wrapping_div(num2);
                self.push_word(res.to_le_bytes());
            }
            OpIDivUDword => {
                let num2 = u32::from_le_bytes(self.pop_dword());
                let num1 = u32::from_le_bytes(self.pop_dword());
                if num2 == 0 {
                    return Err(RuntimeError::new("Division by zero.".to_string()));
                }
                let res = num1.wrapping_div(num2);
                self.push_dword(res.to_le_bytes());
            }
            OpIDivUQword => {
                let num2 = u64::from_le_bytes(self.pop_qword());
                let num1 = u64::from_le_bytes(self.pop_qword());
                if num2 == 0 {
                    return Err(RuntimeError::new("Division by zero.".to_string()));
                }
                let res = num1.wrapping_div(num2);
                self.push_qword(res.to_le_bytes());
            }
            OpIDivUOword => {
                let num2 = u128::from_le_bytes(self.pop_oword());
                let num1 = u128::from_le_bytes(self.pop_oword());
                if num2 == 0 {
                    return Err(RuntimeError::new("Division by zero.".to_string()));
                }
                let res = num1.wrapping_div(num2);
                self.push_oword(res.to_le_bytes());
            }
            OpIModSByte => {
                let num2 = i8::from_le_bytes(self.pop_byte());
                let num1 = i8::from_le_bytes(self.pop_byte());
                if num2 == 0 {
                    return Err(RuntimeError::new("Integer modulo by zero.".to_string()));
                }
                let res = num1 % num2;
                self.push_byte(res.to_le_bytes());
            }
            OpIModSWord => {
                let num2 = i16::from_le_bytes(self.pop_word());
                let num1 = i16::from_le_bytes(self.pop_word());
                if num2 == 0 {
                    return Err(RuntimeError::new("Integer modulo by zero.".to_string()));
                }
                let res = num1 % num2;
                self.push_word(res.to_le_bytes());
            }
            OpIModSDword => {
                let num2 = i32::from_le_bytes(self.pop_dword());
                let num1 = i32::from_le_bytes(self.pop_dword());
                if num2 == 0 {
                    return Err(RuntimeError::new("Integer modulo by zero.".to_string()));
                }
                let res = num1 % num2;
                self.push_dword(res.to_le_bytes());
            }
            OpIModSQword => {
                let num2 = i64::from_le_bytes(self.pop_qword());
                let num1 = i64::from_le_bytes(self.pop_qword());
                if num2 == 0 {
                    return Err(RuntimeError::new("Integer modulo by zero.".to_string()));
                }
                let res = num1 % num2;
                self.push_qword(res.to_le_bytes());
            }
            OpIModSOword => {
                let num2 = i128::from_le_bytes(self.pop_oword());
                let num1 = i128::from_le_bytes(self.pop_oword());
                if num2 == 0 {
                    return Err(RuntimeError::new("Integer modulo by zero.".to_string()));
                }
                let res = num1 % num2;
                self.push_oword(res.to_le_bytes());
            }
            OpIModUByte => {
                let num2 = u8::from_le_bytes(self.pop_byte());
                let num1 = u8::from_le_bytes(self.pop_byte());
                if num2 == 0 {
                    return Err(RuntimeError::new("Integer modulo by zero.".to_string()));
                }
                let res = num1 % num2;
                self.push_byte(res.to_le_bytes());
            }
            OpIModUWord => {
                let num2 = u16::from_le_bytes(self.pop_word());
                let num1 = u16::from_le_bytes(self.pop_word());
                if num2 == 0 {
                    return Err(RuntimeError::new("Integer modulo by zero.".to_string()));
                }
                let res = num1 % num2;
                self.push_word(res.to_le_bytes());
            }
            OpIModUDword => {
                let num2 = u32::from_le_bytes(self.pop_dword());
                let num1 = u32::from_le_bytes(self.pop_dword());
                if num2 == 0 {
                    return Err(RuntimeError::new("Integer modulo by zero.".to_string()));
                }
                let res = num1 % num2;
                self.push_dword(res.to_le_bytes());
            }
            OpIModUQword => {
                let num2 = u64::from_le_bytes(self.pop_qword());
                let num1 = u64::from_le_bytes(self.pop_qword());
                if num2 == 0 {
                    return Err(RuntimeError::new("Integer modulo by zero.".to_string()));
                }
                let res = num1 % num2;
                self.push_qword(res.to_le_bytes());
            }
            OpIModUOword => {
                let num2 = u128::from_le_bytes(self.pop_oword());
                let num1 = u128::from_le_bytes(self.pop_oword());
                if num2 == 0 {
                    return Err(RuntimeError::new("Integer modulo by zero.".to_string()));
                }
                let res = num1 % num2;
                self.push_oword(res.to_le_bytes());
            }
            OpINegByte => {
                let num = i8::from_le_bytes(self.pop_byte());
                let res = -num;
                self.push_byte(res.to_le_bytes());
            }
            OpINegWord => {
                let num = i16::from_le_bytes(self.pop_word());
                let res = -num;
                self.push_word(res.to_le_bytes());
            }
            OpINegDword => {
                let num = i32::from_le_bytes(self.pop_dword());
                let res = -num;
                self.push_dword(res.to_le_bytes());
            }
            OpINegQword => {
                let num = i64::from_le_bytes(self.pop_qword());
                let res = -num;
                self.push_qword(res.to_le_bytes());
            }
            OpINegOword => {
                let num = i128::from_le_bytes(self.pop_oword());
                let res = -num;
                self.push_oword(res.to_le_bytes());
            }
            OpConvertSWordToFloat => {
                let num = i16::from_le_bytes(self.pop_word());
                let res = num as f32;
                self.push_dword(res.to_le_bytes());
            }
            OpConvertUWordToFloat => {
                let num = u16::from_le_bytes(self.pop_word());
                let res = num as f32;
                self.push_dword(res.to_le_bytes());
            }
            OpConvertSQwordToFloat => {
                let num = i64::from_le_bytes(self.pop_qword());
                let res = num as f32;
                self.push_dword(res.to_le_bytes());
            }
            OpConvertUQwordToFloat => {
                let num = u64::from_le_bytes(self.pop_qword());
                let res = num as f32;
                self.push_dword(res.to_le_bytes());
            }
            OpConvertSOwordToFloat => {
                let num = i128::from_le_bytes(self.pop_oword());
                let res = num as f32;
                self.push_dword(res.to_le_bytes());
            }
            OpConvertUOwordToFloat => {
                let num = u128::from_le_bytes(self.pop_oword());
                let res = num as f32;
                self.push_dword(res.to_le_bytes());
            }
            OpConvertSWordToDouble => {
                let num = i16::from_le_bytes(self.pop_word());
                let res = num as f64;
                self.push_qword(res.to_le_bytes());
            }
            OpConvertUWordToDouble => {
                let num = u16::from_le_bytes(self.pop_word());
                let res = num as f64;
                self.push_qword(res.to_le_bytes());
            }
            OpConvertSQwordToDouble => {
                let num = i64::from_le_bytes(self.pop_qword());
                let res = num as f64;
                self.push_qword(res.to_le_bytes());
            }
            OpConvertUQwordToDouble => {
                let num = u64::from_le_bytes(self.pop_qword());
                let res = num as f64;
                self.push_qword(res.to_le_bytes());
            }
            OpConvertSOwordToDouble => {
                let num = i128::from_le_bytes(self.pop_oword());
                let res = num as f64;
                self.push_qword(res.to_le_bytes());
            }
            OpConvertUOwordToDouble => {
                let num = u128::from_le_bytes(self.pop_oword());
                let res = num as f64;
                self.push_qword(res.to_le_bytes());
            }
            OpConvertFloatToSWord => {
                let num = f32::from_le_bytes(self.pop_dword());
                let res = num as i16;
                self.push_word(res.to_le_bytes());
            }
            OpConvertFloatToUWord => {
                let num = f32::from_le_bytes(self.pop_dword());
                let res = num as u16;
                self.push_word(res.to_le_bytes());
            }
            OpConvertFloatToSQword => {
                let num = f32::from_le_bytes(self.pop_dword());
                let res = num as i64;
                self.push_qword(res.to_le_bytes());
            }
            OpConvertFloatToUQword => {
                let num = f32::from_le_bytes(self.pop_dword());
                let res = num as u64;
                self.push_qword(res.to_le_bytes());
            }
            OpConvertFloatToSOword => {
                let num = f32::from_le_bytes(self.pop_dword());
                let res = num as i128;
                self.push_oword(res.to_le_bytes());
            }
            OpConvertFloatToUOword => {
                let num = f32::from_le_bytes(self.pop_dword());
                let res = num as u128;
                self.push_oword(res.to_le_bytes());
            }
            OpConvertDoubleToSWord => {
                let num = f64::from_le_bytes(self.pop_qword());
                let res = num as i16;
                self.push_word(res.to_le_bytes());
            }
            OpConvertDoubleToUWord => {
                let num = f64::from_le_bytes(self.pop_qword());
                let res = num as u16;
                self.push_word(res.to_le_bytes());
            }
            OpConvertDoubleToSQword => {
                let num = f64::from_le_bytes(self.pop_qword());
                let res = num as i64;
                self.push_qword(res.to_le_bytes());
            }
            OpConvertDoubleToUQword => {
                let num = f64::from_le_bytes(self.pop_qword());
                let res = num as u64;
                self.push_qword(res.to_le_bytes());
            }
            OpConvertDoubleToSOword => {
                let num = f64::from_le_bytes(self.pop_qword());
                let res = num as i128;
                self.push_oword(res.to_le_bytes());
            }
            OpConvertDoubleToUOword => {
                let num = f64::from_le_bytes(self.pop_qword());
                let res = num as u128;
                self.push_oword(res.to_le_bytes());
            }
            OpConvertFloatToDouble => {
                let num = f32::from_le_bytes(self.pop_dword());
                let res = num as f64;
                self.push_qword(res.to_le_bytes());
            }
            OpConvertDoubleToFloat => {
                let num = f64::from_le_bytes(self.pop_qword());
                let res = num as f32;
                self.push_dword(res.to_le_bytes());
            }
            OpConvertByteToBool => {
                let num = u8::from_le_bytes(self.pop_byte());
                self.push_bool(num != 0);
            }
            OpConvertWordToBool => {
                let num = u16::from_le_bytes(self.pop_word());
                self.push_bool(num != 0);
            }
            OpConvertDwordToBool => {
                let num = u32::from_le_bytes(self.pop_dword());
                self.push_bool(num != 0);
            }
            OpConvertQwordToBool => {
                let num = u64::from_le_bytes(self.pop_qword());
                self.push_bool(num != 0);
            }
            OpConvertOwordToBool => {
                let num = u128::from_le_bytes(self.pop_oword());
                self.push_bool(num != 0);
            }
            OpFAddFloat => {
                let num2 = f32::from_le_bytes(self.pop_dword());
                let num1 = f32::from_le_bytes(self.pop_dword());
                let res = num1 + num2;
                self.push_dword(res.to_le_bytes());
            }
            OpFAddDouble => {
                let num2 = f64::from_le_bytes(self.pop_qword());
                let num1 = f64::from_le_bytes(self.pop_qword());
                let res = num1 + num2;
                self.push_qword(res.to_le_bytes());
            }
            OpFSubFloat => {
                let num2 = f32::from_le_bytes(self.pop_dword());
                let num1 = f32::from_le_bytes(self.pop_dword());
                let res = num1 - num2;
                self.push_dword(res.to_le_bytes());
            }
            OpFSubDouble => {
                let num2 = f64::from_le_bytes(self.pop_qword());
                let num1 = f64::from_le_bytes(self.pop_qword());
                let res = num1 - num2;
                self.push_qword(res.to_le_bytes());
            }
            OpFMulFloat => {
                let num2 = f32::from_le_bytes(self.pop_dword());
                let num1 = f32::from_le_bytes(self.pop_dword());
                let res = num1 * num2;
                self.push_dword(res.to_le_bytes());
            }
            OpFMulDouble => {
                let num2 = f64::from_le_bytes(self.pop_qword());
                let num1 = f64::from_le_bytes(self.pop_qword());
                let res = num1 * num2;
                self.push_qword(res.to_le_bytes());
            }
            OpFDivFloat => {
                let num2 = f32::from_le_bytes(self.pop_dword());
                let num1 = f32::from_le_bytes(self.pop_dword());
                if num2 == 0.0 {
                    return Err(RuntimeError::new("Division by zero.".to_string()));
                }
                let res = num1 / num2;
                self.push_dword(res.to_le_bytes());
            }
            OpFDivDouble => {
                let num2 = f64::from_le_bytes(self.pop_qword());
                let num1 = f64::from_le_bytes(self.pop_qword());
                if num2 == 0.0 {
                    return Err(RuntimeError::new("Division by zero.".to_string()));
                }
                let res = num1 / num2;
                self.push_qword(res.to_le_bytes());
            }
            OpFNegFloat => {
                let num = f32::from_le_bytes(self.pop_dword());
                let res = -num;
                self.push_dword(res.to_le_bytes());
            }
            OpFNegDouble => {
                let num = f64::from_le_bytes(self.pop_qword());
                let res = -num;
                self.push_qword(res.to_le_bytes());
            }
            OpBitNotByte => {
                let num = u8::from_le_bytes(self.pop_byte());
                let res = !num;
                self.push_byte(res.to_le_bytes());
            }
            OpBitNotWord => {
                let num = u16::from_le_bytes(self.pop_word());
                let res = !num;
                self.push_word(res.to_le_bytes());
            }
            OpBitNotDword => {
                let num = u32::from_le_bytes(self.pop_dword());
                let res = !num;
                self.push_dword(res.to_le_bytes());
            }
            OpBitNotQword => {
                let num = u64::from_le_bytes(self.pop_qword());
                let res = !num;
                self.push_qword(res.to_le_bytes());
            }
            OpBitNotOword => {
                let num = u128::from_le_bytes(self.pop_oword());
                let res = !num;
                self.push_oword(res.to_le_bytes());
            }
            OpBitAndByte => {
                let num2 = u8::from_le_bytes(self.pop_byte());
                let num1 = u8::from_le_bytes(self.pop_byte());
                let res = num1 & num2;
                self.push_byte(res.to_le_bytes());
            }
            OpBitAndWord => {
                let num2 = u16::from_le_bytes(self.pop_word());
                let num1 = u16::from_le_bytes(self.pop_word());
                let res = num1 & num2;
                self.push_word(res.to_le_bytes());
            }
            OpBitAndDword => {
                let num2 = u32::from_le_bytes(self.pop_dword());
                let num1 = u32::from_le_bytes(self.pop_dword());
                let res = num1 & num2;
                self.push_dword(res.to_le_bytes());
            }
            OpBitAndQword => {
                let num2 = u64::from_le_bytes(self.pop_qword());
                let num1 = u64::from_le_bytes(self.pop_qword());
                let res = num1 & num2;
                self.push_qword(res.to_le_bytes());
            }
            OpBitAndOword => {
                let num2 = u128::from_le_bytes(self.pop_oword());
                let num1 = u128::from_le_bytes(self.pop_oword());
                let res = num1 & num2;
                self.push_oword(res.to_le_bytes());
            }
            OpBitOrByte => {
                let num2 = u8::from_le_bytes(self.pop_byte());
                let num1 = u8::from_le_bytes(self.pop_byte());
                let res = num1 | num2;
                self.push_byte(res.to_le_bytes());
            }
            OpBitOrWord => {
                let num2 = u16::from_le_bytes(self.pop_word());
                let num1 = u16::from_le_bytes(self.pop_word());
                let res = num1 | num2;
                self.push_word(res.to_le_bytes());
            }
            OpBitOrDword => {
                let num2 = u32::from_le_bytes(self.pop_dword());
                let num1 = u32::from_le_bytes(self.pop_dword());
                let res = num1 | num2;
                self.push_dword(res.to_le_bytes());
            }
            OpBitOrQword => {
                let num2 = u64::from_le_bytes(self.pop_qword());
                let num1 = u64::from_le_bytes(self.pop_qword());
                let res = num1 | num2;
                self.push_qword(res.to_le_bytes());
            }
            OpBitOrOword => {
                let num2 = u128::from_le_bytes(self.pop_oword());
                let num1 = u128::from_le_bytes(self.pop_oword());
                let res = num1 | num2;
                self.push_oword(res.to_le_bytes());
            }
            OpBitXorByte => {
                let num2 = u8::from_le_bytes(self.pop_byte());
                let num1 = u8::from_le_bytes(self.pop_byte());
                let res = num1 ^ num2;
                self.push_byte(res.to_le_bytes());
            }
            OpBitXorWord => {
                let num2 = u16::from_le_bytes(self.pop_word());
                let num1 = u16::from_le_bytes(self.pop_word());
                let res = num1 ^ num2;
                self.push_word(res.to_le_bytes());
            }
            OpBitXorDword => {
                let num2 = u32::from_le_bytes(self.pop_dword());
                let num1 = u32::from_le_bytes(self.pop_dword());
                let res = num1 ^ num2;
                self.push_dword(res.to_le_bytes());
            }
            OpBitXorQword => {
                let num2 = u64::from_le_bytes(self.pop_qword());
                let num1 = u64::from_le_bytes(self.pop_qword());
                let res = num1 ^ num2;
                self.push_qword(res.to_le_bytes());
            }
            OpBitXorOword => {
                let num2 = u128::from_le_bytes(self.pop_oword());
                let num1 = u128::from_le_bytes(self.pop_oword());
                let res = num1 ^ num2;
                self.push_oword(res.to_le_bytes());
            }
            OpShiftLeftByte => {
                let num = u8::from_le_bytes(self.pop_byte());
                let byte = u8::from_le_bytes(self.pop_byte());
                let res = byte.wrapping_shl(num as u32);
                self.push_byte(res.to_le_bytes());
            }
            OpShiftLeftWord => {
                let num = u8::from_le_bytes(self.pop_byte());
                let word = u16::from_le_bytes(self.pop_word());
                let res = word.wrapping_shl(num as u32);
                self.push_word(res.to_le_bytes());
            }
            OpShiftLeftDword => {
                let num = u8::from_le_bytes(self.pop_byte());
                let dword = u32::from_le_bytes(self.pop_dword());
                let res = dword.wrapping_shl(num as u32);
                self.push_dword(res.to_le_bytes());
            }
            OpShiftLeftQword => {
                let num = u8::from_le_bytes(self.pop_byte());
                let qword = u64::from_le_bytes(self.pop_qword());
                let res = qword.wrapping_shl(num as u32);
                self.push_qword(res.to_le_bytes());
            }
            OpShiftLeftOword => {
                let num = u8::from_le_bytes(self.pop_byte());
                let oword = u128::from_le_bytes(self.pop_oword());
                let res = oword.wrapping_shl(num as u32);
                self.push_oword(res.to_le_bytes());
            }
            OpSignShiftRightByte => {
                let num = u8::from_le_bytes(self.pop_byte());
                let byte = i8::from_le_bytes(self.pop_byte());
                let res = byte.wrapping_shr(num as u32);
                self.push_byte(res.to_le_bytes());
            }
            OpSignShiftRightWord => {
                let num = u8::from_le_bytes(self.pop_byte());
                let word = i16::from_le_bytes(self.pop_word());
                let res = word.wrapping_shr(num as u32);
                self.push_word(res.to_le_bytes());
            }
            OpSignShiftRightDword => {
                let num = u8::from_le_bytes(self.pop_byte());
                let dword = i32::from_le_bytes(self.pop_dword());
                let res = dword.wrapping_shr(num as u32);
                self.push_dword(res.to_le_bytes());
            }
            OpSignShiftRightQword => {
                let num = u8::from_le_bytes(self.pop_byte());
                let qword = i64::from_le_bytes(self.pop_qword());
                let res = qword.wrapping_shr(num as u32);
                self.push_qword(res.to_le_bytes());
            }
            OpSignShiftRightOword => {
                let num = u8::from_le_bytes(self.pop_byte());
                let oword = i128::from_le_bytes(self.pop_oword());
                let res = oword.wrapping_shr(num as u32);
                self.push_oword(res.to_le_bytes());
            }
            OpZeroShiftRightByte => {
                let num = u8::from_le_bytes(self.pop_byte());
                let byte = u8::from_le_bytes(self.pop_byte());
                let res = byte.wrapping_shr(num as u32);
                self.push_byte(res.to_le_bytes());
            }
            OpZeroShiftRightWord => {
                let num = u8::from_le_bytes(self.pop_byte());
                let word = u16::from_le_bytes(self.pop_word());
                let res = word.wrapping_shr(num as u32);
                self.push_word(res.to_le_bytes());
            }
            OpZeroShiftRightDword => {
                let num = u8::from_le_bytes(self.pop_byte());
                let dword = u32::from_le_bytes(self.pop_dword());
                let res = dword.wrapping_shr(num as u32);
                self.push_dword(res.to_le_bytes());
            }
            OpZeroShiftRightQword => {
                let num = u8::from_le_bytes(self.pop_byte());
                let qword = u64::from_le_bytes(self.pop_qword());
                let res = qword.wrapping_shr(num as u32);
                self.push_qword(res.to_le_bytes());
            }
            OpZeroShiftRightOword => {
                let num = u8::from_le_bytes(self.pop_byte());
                let oword = u128::from_le_bytes(self.pop_oword());
                let res = oword.wrapping_shr(num as u32);
                self.push_oword(res.to_le_bytes());
            }
            OpICmpEqualByte => {
                let num2 = u8::from_le_bytes(self.pop_byte());
                let num1 = u8::from_le_bytes(self.pop_byte());
                self.push_bool(num1 == num2);
            }
            OpICmpEqualWord => {
                let num2 = u16::from_le_bytes(self.pop_word());
                let num1 = u16::from_le_bytes(self.pop_word());
                self.push_bool(num1 == num2);
            }
            OpICmpEqualDword => {
                let num2 = u32::from_le_bytes(self.pop_dword());
                let num1 = u32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 == num2);
            }
            OpICmpEqualQword => {
                let num2 = u64::from_le_bytes(self.pop_qword());
                let num1 = u64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 == num2);
            }
            OpICmpEqualOword => {
                let num2 = u128::from_le_bytes(self.pop_oword());
                let num1 = u128::from_le_bytes(self.pop_oword());
                self.push_bool(num1 == num2);
            }
            OpICmpNotEqualByte => {
                let num2 = u8::from_le_bytes(self.pop_byte());
                let num1 = u8::from_le_bytes(self.pop_byte());
                self.push_bool(num1 != num2);
            }
            OpICmpNotEqualWord => {
                let num2 = u16::from_le_bytes(self.pop_word());
                let num1 = u16::from_le_bytes(self.pop_word());
                self.push_bool(num1 != num2);
            }
            OpICmpNotEqualDword => {
                let num2 = u32::from_le_bytes(self.pop_dword());
                let num1 = u32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 != num2);
            }
            OpICmpNotEqualQword => {
                let num2 = u64::from_le_bytes(self.pop_qword());
                let num1 = u64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 != num2);
            }
            OpICmpNotEqualOword => {
                let num2 = u128::from_le_bytes(self.pop_oword());
                let num1 = u128::from_le_bytes(self.pop_oword());
                self.push_bool(num1 != num2);
            }
            OpICmpLessSByte => {
                let num2 = i8::from_le_bytes(self.pop_byte());
                let num1 = i8::from_le_bytes(self.pop_byte());
                self.push_bool(num1 < num2);
            }
            OpICmpLessSWord => {
                let num2 = i16::from_le_bytes(self.pop_word());
                let num1 = i16::from_le_bytes(self.pop_word());
                self.push_bool(num1 < num2);
            }
            OpICmpLessSDword => {
                let num2 = i32::from_le_bytes(self.pop_dword());
                let num1 = i32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 < num2);
            }
            OpICmpLessSQword => {
                let num2 = i64::from_le_bytes(self.pop_qword());
                let num1 = i64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 < num2);
            }
            OpICmpLessSOword => {
                let num2 = i128::from_le_bytes(self.pop_oword());
                let num1 = i128::from_le_bytes(self.pop_oword());
                self.push_bool(num1 < num2);
            }
            OpICmpLessUByte => {
                let num2 = u8::from_le_bytes(self.pop_byte());
                let num1 = u8::from_le_bytes(self.pop_byte());
                self.push_bool(num1 < num2);
            }
            OpICmpLessUWord => {
                let num2 = u16::from_le_bytes(self.pop_word());
                let num1 = u16::from_le_bytes(self.pop_word());
                self.push_bool(num1 < num2);
            }
            OpICmpLessUDword => {
                let num2 = u32::from_le_bytes(self.pop_dword());
                let num1 = u32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 < num2);
            }
            OpICmpLessUQword => {
                let num2 = u64::from_le_bytes(self.pop_qword());
                let num1 = u64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 < num2);
            }
            OpICmpLessUOword => {
                let num2 = u128::from_le_bytes(self.pop_oword());
                let num1 = u128::from_le_bytes(self.pop_oword());
                self.push_bool(num1 < num2);
            }
            OpICmpLessEqualSByte => {
                let num2 = i8::from_le_bytes(self.pop_byte());
                let num1 = i8::from_le_bytes(self.pop_byte());
                self.push_bool(num1 <= num2);
            }
            OpICmpLessEqualSWord => {
                let num2 = i16::from_le_bytes(self.pop_word());
                let num1 = i16::from_le_bytes(self.pop_word());
                self.push_bool(num1 <= num2);
            }
            OpICmpLessEqualSDword => {
                let num2 = i32::from_le_bytes(self.pop_dword());
                let num1 = i32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 <= num2);
            }
            OpICmpLessEqualSQword => {
                let num2 = i64::from_le_bytes(self.pop_qword());
                let num1 = i64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 <= num2);
            }
            OpICmpLessEqualSOword => {
                let num2 = i128::from_le_bytes(self.pop_oword());
                let num1 = i128::from_le_bytes(self.pop_oword());
                self.push_bool(num1 <= num2);
            }
            OpICmpLessEqualUByte => {
                let num2 = u8::from_le_bytes(self.pop_byte());
                let num1 = u8::from_le_bytes(self.pop_byte());
                self.push_bool(num1 <= num2);
            }
            OpICmpLessEqualUWord => {
                let num2 = u16::from_le_bytes(self.pop_word());
                let num1 = u16::from_le_bytes(self.pop_word());
                self.push_bool(num1 <= num2);
            }
            OpICmpLessEqualUDword => {
                let num2 = u32::from_le_bytes(self.pop_dword());
                let num1 = u32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 <= num2);
            }
            OpICmpLessEqualUQword => {
                let num2 = u64::from_le_bytes(self.pop_qword());
                let num1 = u64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 <= num2);
            }
            OpICmpLessEqualUOword => {
                let num2 = u128::from_le_bytes(self.pop_oword());
                let num1 = u128::from_le_bytes(self.pop_oword());
                self.push_bool(num1 <= num2);
            }
            OpICmpGreaterSByte => {
                let num2 = i8::from_le_bytes(self.pop_byte());
                let num1 = i8::from_le_bytes(self.pop_byte());
                self.push_bool(num1 > num2);
            }
            OpICmpGreaterSWord => {
                let num2 = i16::from_le_bytes(self.pop_word());
                let num1 = i16::from_le_bytes(self.pop_word());
                self.push_bool(num1 > num2);
            }
            OpICmpGreaterSDword => {
                let num2 = i32::from_le_bytes(self.pop_dword());
                let num1 = i32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 > num2);
            }
            OpICmpGreaterSQword => {
                let num2 = i64::from_le_bytes(self.pop_qword());
                let num1 = i64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 > num2);
            }
            OpICmpGreaterSOword => {
                let num2 = i128::from_le_bytes(self.pop_oword());
                let num1 = i128::from_le_bytes(self.pop_oword());
                self.push_bool(num1 > num2);
            }
            OpICmpGreaterUByte => {
                let num2 = u8::from_le_bytes(self.pop_byte());
                let num1 = u8::from_le_bytes(self.pop_byte());
                self.push_bool(num1 > num2);
            }
            OpICmpGreaterUWord => {
                let num2 = u16::from_le_bytes(self.pop_word());
                let num1 = u16::from_le_bytes(self.pop_word());
                self.push_bool(num1 > num2);
            }
            OpICmpGreaterUDword => {
                let num2 = u32::from_le_bytes(self.pop_dword());
                let num1 = u32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 > num2);
            }
            OpICmpGreaterUQword => {
                let num2 = u64::from_le_bytes(self.pop_qword());
                let num1 = u64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 > num2);
            }
            OpICmpGreaterUOword => {
                let num2 = u128::from_le_bytes(self.pop_oword());
                let num1 = u128::from_le_bytes(self.pop_oword());
                self.push_bool(num1 > num2);
            }
            OpICmpGreaterEqualSByte => {
                let num2 = i8::from_le_bytes(self.pop_byte());
                let num1 = i8::from_le_bytes(self.pop_byte());
                self.push_bool(num1 >= num2);
            }
            OpICmpGreaterEqualSWord => {
                let num2 = i16::from_le_bytes(self.pop_word());
                let num1 = i16::from_le_bytes(self.pop_word());
                self.push_bool(num1 >= num2);
            }
            OpICmpGreaterEqualSDword => {
                let num2 = i32::from_le_bytes(self.pop_dword());
                let num1 = i32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 >= num2);
            }
            OpICmpGreaterEqualSQword => {
                let num2 = i64::from_le_bytes(self.pop_qword());
                let num1 = i64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 >= num2);
            }
            OpICmpGreaterEqualSOword => {
                let num2 = i128::from_le_bytes(self.pop_oword());
                let num1 = i128::from_le_bytes(self.pop_oword());
                self.push_bool(num1 >= num2);
            }
            OpICmpGreaterEqualUByte => {
                let num2 = u8::from_le_bytes(self.pop_byte());
                let num1 = u8::from_le_bytes(self.pop_byte());
                self.push_bool(num1 >= num2);
            }
            OpICmpGreaterEqualUWord => {
                let num2 = u16::from_le_bytes(self.pop_word());
                let num1 = u16::from_le_bytes(self.pop_word());
                self.push_bool(num1 >= num2);
            }
            OpICmpGreaterEqualUDword => {
                let num2 = u32::from_le_bytes(self.pop_dword());
                let num1 = u32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 >= num2);
            }
            OpICmpGreaterEqualUQword => {
                let num2 = u64::from_le_bytes(self.pop_qword());
                let num1 = u64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 >= num2);
            }
            OpICmpGreaterEqualUOword => {
                let num2 = u128::from_le_bytes(self.pop_oword());
                let num1 = u128::from_le_bytes(self.pop_oword());
                self.push_bool(num1 >= num2);
            }
            OpFCmpEqualFloat => {
                let num2 = f32::from_le_bytes(self.pop_dword());
                let num1 = f32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 == num2);
            }
            OpFCmpEqualDouble => {
                let num2 = f64::from_le_bytes(self.pop_qword());
                let num1 = f64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 == num2);
            }
            OpFCmpNotEqualFloat => {
                let num2 = f32::from_le_bytes(self.pop_dword());
                let num1 = f32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 != num2);
            }
            OpFCmpNotEqualDouble => {
                let num2 = f64::from_le_bytes(self.pop_qword());
                let num1 = f64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 != num2);
            }
            OpFCmpLessFloat => {
                let num2 = f32::from_le_bytes(self.pop_dword());
                let num1 = f32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 < num2);
            }
            OpFCmpLessDouble => {
                let num2 = f64::from_le_bytes(self.pop_qword());
                let num1 = f64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 < num2);
            }
            OpFCmpLessEqualFloat => {
                let num2 = f32::from_le_bytes(self.pop_dword());
                let num1 = f32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 <= num2);
            }
            OpFCmpLessEqualDouble => {
                let num2 = f64::from_le_bytes(self.pop_qword());
                let num1 = f64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 <= num2);
            }
            OpFCmpGreaterFloat => {
                let num2 = f32::from_le_bytes(self.pop_dword());
                let num1 = f32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 > num2);
            }
            OpFCmpGreaterDouble => {
                let num2 = f64::from_le_bytes(self.pop_qword());
                let num1 = f64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 > num2);
            }
            OpFCmpGreaterEqualFloat => {
                let num2 = f32::from_le_bytes(self.pop_dword());
                let num1 = f32::from_le_bytes(self.pop_dword());
                self.push_bool(num1 >= num2);
            }
            OpFCmpGreaterEqualDouble => {
                let num2 = f64::from_le_bytes(self.pop_qword());
                let num1 = f64::from_le_bytes(self.pop_qword());
                self.push_bool(num1 >= num2);
            }
            OpPopByte => {
                self.pop_byte();
            }
            OpPopWord => {
                self.pop_word();
            }
            OpPopDword => {
                self.pop_dword();
            }
            OpPopQword => {
                self.pop_qword();
            }
            OpPopOword => {
                self.pop_oword();
            }
            OpPushByte => {
                let byte = self.read_arg_byte();
                self.push_byte(byte);
            }
            OpPushWord => {
                let word = self.read_arg_word();
                self.push_word(word);
            }
            OpPushDword => {
                let dword = self.read_arg_dword();
                self.push_dword(dword);
            }
            OpPushQword => {
                let qword = self.read_arg_qword();
                self.push_qword(qword);
            }
            OpPushOword => {
                let oword = self.read_arg_oword();
                self.push_oword(oword);
            }
            OpCopyByte => {
                let byte = self.peek_byte();
                self.push_byte(byte);
            }
            OpCopyWord => {
                let word = self.peek_word();
                self.push_word(word);
            }
            OpCopyDword => {
                let dword = self.peek_dword();
                self.push_dword(dword);
            }
            OpCopyQword => {
                let qword = self.peek_qword();
                self.push_qword(qword);
            }
            OpCopyOword => {
                let oword = self.peek_oword();
                self.push_oword(oword);
            }
            OpGetLocalByte => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let byte = self.get_frame_slot_byte(slot as usize);
                self.push_byte(byte);
            }
            OpGetLocalWord => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let word = self.get_frame_slot_word(slot as usize);
                self.push_word(word);
            }
            OpGetLocalDword => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let dword = self.get_frame_slot_dword(slot as usize);
                self.push_dword(dword);
            }
            OpGetLocalQword => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let qword = self.get_frame_slot_qword(slot as usize);
                self.push_qword(qword);
            }
            OpGetLocalOword => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let oword = self.get_frame_slot_oword(slot as usize);
                self.push_oword(oword);
            }
            OpSetLocalByte => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let byte = self.pop_byte();
                self.set_frame_slot_byte(slot as usize, byte);
            }
            OpSetLocalWord => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let word = self.pop_word();
                self.set_frame_slot_word(slot as usize, word);
            }
            OpSetLocalDword => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let dword = self.pop_dword();
                self.set_frame_slot_dword(slot as usize, dword);
            }
            OpSetLocalQword => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let qword = self.pop_qword();
                self.set_frame_slot_qword(slot as usize, qword);
            }
            OpSetLocalOword => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let oword = self.pop_oword();
                self.set_frame_slot_oword(slot as usize, oword);
            }
            OpGetReferenceByte => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let ref_slot = u32::from_le_bytes(self.get_frame_slot_dword(slot as usize));
                let byte = self.get_frame_slot_byte(ref_slot as usize);
                self.push_byte(byte);
            }
            OpGetReferenceWord => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let ref_slot = u32::from_le_bytes(self.get_frame_slot_dword(slot as usize));
                let word = self.get_frame_slot_word(ref_slot as usize);
                self.push_word(word);
            }
            OpGetReferenceDword => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let ref_slot = u32::from_le_bytes(self.get_frame_slot_dword(slot as usize));
                let dword = self.get_frame_slot_dword(ref_slot as usize);
                self.push_dword(dword);
            }
            OpGetReferenceQword => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let ref_slot = u32::from_le_bytes(self.get_frame_slot_dword(slot as usize));
                let qword = self.get_frame_slot_qword(ref_slot as usize);
                self.push_qword(qword);
            }
            OpGetReferenceOword => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let ref_slot = u32::from_le_bytes(self.get_frame_slot_dword(slot as usize));
                let oword = self.get_frame_slot_oword(ref_slot as usize);
                self.push_oword(oword);
            }
            OpSetReferenceByte => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let ref_slot = u32::from_le_bytes(self.get_frame_slot_dword(slot as usize));
                let byte = self.pop_byte();
                self.set_frame_slot_byte(ref_slot as usize, byte);
            }
            OpSetReferenceWord => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let ref_slot = u32::from_le_bytes(self.get_frame_slot_dword(slot as usize));
                let word = self.pop_word();
                self.set_frame_slot_word(ref_slot as usize, word);
            }
            OpSetReferenceDword => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let ref_slot = u32::from_le_bytes(self.get_frame_slot_dword(slot as usize));
                let dword = self.pop_dword();
                self.set_frame_slot_dword(ref_slot as usize, dword);
            }
            OpSetReferenceQword => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let ref_slot = u32::from_le_bytes(self.get_frame_slot_dword(slot as usize));
                let qword = self.pop_qword();
                self.set_frame_slot_qword(ref_slot as usize, qword);
            }
            OpSetReferenceOword => {
                let slot = u32::from_le_bytes(self.read_arg_dword());
                let ref_slot = u32::from_le_bytes(self.get_frame_slot_dword(slot as usize));
                let oword = self.pop_oword();
                self.set_frame_slot_oword(ref_slot as usize, oword);
            }
        }

        return Ok(());
    }
    
    /// 运行特殊功能
    #[inline]
    fn run_special_function(&mut self, special_func: SpecialFunction) -> RuntimeResult<()> {
        use crate::instr::SpecialFunction::*;
        match special_func {
            PrintByte => {
                let byte = u8::from_le_bytes(self.pop_byte());
                Self::stdout_print_byte(byte);
            }
            PrintSByte => {
                let sbyte = i8::from_le_bytes(self.pop_byte());
                Self::stdout_print_sbyte(sbyte);
            }
            PrintShort => {
                let short = i16::from_le_bytes(self.pop_word());
                Self::stdout_print_short(short);
            }
            PrintUShort => {
                let ushort = u16::from_le_bytes(self.pop_word());
                Self::stdout_print_ushort(ushort);
            }
            PrintInt => {
                let int = i32::from_le_bytes(self.pop_dword());
                Self::stdout_print_int(int);
            }
            PrintUInt => {
                let uint = u32::from_le_bytes(self.pop_dword());
                Self::stdout_print_uint(uint);
            }
            PrintLong => {
                let long = i64::from_le_bytes(self.pop_qword());
                Self::stdout_print_long(long);
            }
            PrintULong => {
                let ulong = u64::from_le_bytes(self.pop_qword());
                Self::stdout_print_ulong(ulong);
            }
            PrintExtInt => {
                let extint = i128::from_le_bytes(self.pop_oword());
                Self::stdout_print_extint(extint);
            }
            PrintUExtInt => {
                let uextint = u128::from_le_bytes(self.pop_oword());
                Self::stdout_print_uextint(uextint);
            }
            PrintFloat => {
                let float = f32::from_le_bytes(self.pop_dword());
                Self::stdout_print_float(float);
            }
            PrintDouble => {
                let double = f64::from_le_bytes(self.pop_qword());
                Self::stdout_print_double(double);
            }
            PrintBool => {
                let value = self.pop_bool();
                Self::stdout_print_bool(value);
            }
            PrintChar => {
                #[cfg(debug_assertions)]
                {
                    let ch = if let Some(temp) = char::from_u32(u32::from_le_bytes(self.pop_dword())) {
                        temp
                    } else {
                        panic!("Invalid Unicode point code.")
                    };
                    Self::stdout_print_char(ch);
                }
                #[cfg(not(debug_assertions))]
                {
                    let ch = char::from_u32(u32::from_le_bytes(self.pop_dword())).unwrap();
                    Self::stdout_print_char(ch);
                }
            }
            PrintNewLine => {
                Self::stdout_print_new_line();
            }
        }
        
        return Ok(());
    }
}
