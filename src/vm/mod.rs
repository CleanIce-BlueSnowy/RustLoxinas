//! 虚拟机模块

use crate::byte_handler::byte_reader::{read_byte, read_dword, read_extend, read_qword, read_word};
use crate::disassembler::disassemble_instruction;
use crate::instr::Instruction;
use crate::instr::Instruction::*;

mod vm_assistance;
mod vm_debug;

pub struct VM<'a> {
    pub vm_stack: Vec<u8>,
    pub chunk: &'a [u8],
    pub ip: usize,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a [u8]) -> Self {
        Self { vm_stack: Vec::new(), chunk, ip: 0 }
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        while self.ip < self.chunk.len() {
            #[cfg(debug_assertions)]
            let old_ip = self.ip;

            let instr_byte;
            match read_byte(self.chunk, self.ip) {
                Ok(temp) => instr_byte = temp.0[0],
                Err(_) => panic!("Unexpected error!"),
            }

            self.ip += 1;

            let instr;
            match Instruction::try_from(instr_byte) {
                Ok(temp) => instr = temp,
                Err(_) => return Err(RuntimeError::new(format!("Unknown instruction: {:02x}", instr_byte))),
            }

            #[cfg(debug_assertions)]
            {
                self.print_stack();
                match disassemble_instruction(instr.clone(), self.chunk, old_ip + 1) {
                    Ok(temp) => println!("{}", temp.0),
                    Err(err) => return Err(RuntimeError::new(format!("Disassembler threw an error: {}", err))),
                }
                println!();
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

    fn run_code(&mut self, instr: Instruction) -> Result<(), RuntimeError> {
        match instr {
            OpReturn => {
                return Err(RuntimeError::new("Return instruction".to_string()));
            }
            OpLoadConstByte => {
                if let Ok((byte, new_ip)) = read_byte(self.chunk, self.ip) {
                    self.ip = new_ip;
                    self.push_byte(byte);
                } else {
                    return Err(RuntimeError::new("Data not enough: need 1 byte.".to_string()));
                }
            }
            OpLoadConstWord => {
                if let Ok((word, new_ip)) = read_word(self.chunk, self.ip) {
                    self.ip = new_ip;
                    self.push_word(word);
                } else {
                    return Err(RuntimeError::new("Data not enough: need 2 bytes.".to_string()));
                }
            }
            OpLoadConstDword => {
                if let Ok((dword, new_ip)) = read_dword(self.chunk, self.ip) {
                    self.ip = new_ip;
                    self.push_dword(dword);
                } else {
                    return Err(RuntimeError::new("Data not enough: need 4 bytes.".to_string()));
                }
            }
            OpLoadConstQword => {
                if let Ok((qword, new_ip)) = read_qword(self.chunk, self.ip) {
                    self.ip = new_ip;
                    self.push_qword(qword);
                } else {
                    return Err(RuntimeError::new("Data not enough: need 8 bytes.".to_string()));
                }
            }
            OpLoadConstExtInt => {
                if let Ok((extend, new_ip)) = read_extend(self.chunk, self.ip) {
                    self.ip = new_ip;
                    self.push_extend(extend);
                } else {
                    return Err(RuntimeError::new("Data not enough: need 16 bytes.".to_string()));
                }
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
            OpZeroExtendByteToWord => {
                self.push_byte([0x00]);
            }
            OpZeroExtendWordToDword => {
                self.push_word([0x00, 0x00]);
            }
            OpZeroExtendDwordToQword => {
                self.push_dword([0x00, 0x00, 0x00, 0x00]);
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
            OpSignExtendToExtInt => {
                let high_byte = self.peek_byte()[0];
                if high_byte & 0b10000000 == 0 {
                    self.push_qword([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
                } else {
                    self.push_qword([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
                }
            }
            OpZeroExtendToExtInt => {
                self.push_qword([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
            }
            OpTruncateFromExtInt => {
                self.pop_qword();
            }
            OpIAddExtInt => {
                let num2 = u128::from_le_bytes(self.pop_extend());
                let num1 = u128::from_le_bytes(self.pop_extend());
                let res = num1.wrapping_add(num2);
                self.push_extend(res.to_le_bytes());
            }
            OpISubExtInt => {
                let num2 = u128::from_le_bytes(self.pop_extend());
                let num1 = u128::from_le_bytes(self.pop_extend());
                let res = num1.wrapping_sub(num2);
                self.push_extend(res.to_le_bytes());
            }
            OpIMulExtInt => {
                let num2 = u128::from_le_bytes(self.pop_extend());
                let num1 = u128::from_le_bytes(self.pop_extend());
                let res = num1.wrapping_mul(num2);
                self.push_extend(res.to_le_bytes());
            }
            OpIDivSExtInt => {
                let num2 = i128::from_le_bytes(self.pop_extend());
                let num1 = i128::from_le_bytes(self.pop_extend());
                if num2 == 0 {
                    return Err(RuntimeError::new("Division by zero.".to_string()));
                }
                let res = num1.wrapping_div(num2);
                self.push_extend(res.to_le_bytes());
            }
            OpIDivUExtInt => {
                let num2 = u128::from_le_bytes(self.pop_extend());
                let num1 = u128::from_le_bytes(self.pop_extend());
                if num2 == 0 {
                    return Err(RuntimeError::new("Division by zero.".to_string()));
                }
                let res = num1.wrapping_div(num2);
                self.push_extend(res.to_le_bytes());
            }
            OpIModSExtInt => {
                let num2 = i128::from_le_bytes(self.pop_extend());
                let num1 = i128::from_le_bytes(self.pop_extend());
                if num2 == 0 {
                    return Err(RuntimeError::new("Integer modulo by zero.".to_string()));
                }
                let res = num1 % num2;
                self.push_extend(res.to_le_bytes());
            }
            OpIModUExtInt => {
                let num2 = u128::from_le_bytes(self.pop_extend());
                let num1 = u128::from_le_bytes(self.pop_extend());
                if num2 == 0 {
                    return Err(RuntimeError::new("Integer modulo by zero.".to_string()));
                }
                let res = num1 % num2;
                self.push_extend(res.to_le_bytes());
            }
            OpINegExtInt => {
                let num = i128::from_le_bytes(self.pop_extend());
                let res = -num;
                self.push_extend(res.to_le_bytes());
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
            OpConvertSExtIntToFloat => {
                let num = i128::from_le_bytes(self.pop_extend());
                let res = num as f32;
                self.push_dword(res.to_le_bytes());
            }
            OpConvertUExtIntToFloat => {
                let num = u128::from_le_bytes(self.pop_extend());
                let res = num as f32;
                self.push_dword(res.to_le_bytes());
            }
            OpConvertSExtIntToDouble => {
                let num = i128::from_le_bytes(self.pop_extend());
                let res = num as f64;
                self.push_qword(res.to_le_bytes());
            }
            OpConvertUExtIntToDouble => {
                let num = u128::from_le_bytes(self.pop_extend());
                let res = num as f64;
                self.push_qword(res.to_le_bytes());
            }
            OpConvertFloatToSExtInt => {
                let num = f32::from_le_bytes(self.pop_dword());
                let res = num as i128;
                self.push_extend(res.to_le_bytes());
            }
            OpConvertFloatToUExtInt => {
                let num = f32::from_le_bytes(self.pop_dword());
                let res = num as u128;
                self.push_extend(res.to_le_bytes());
            }
            OpConvertDoubleToSExtInt => {
                let num = f64::from_le_bytes(self.pop_qword());
                let res = num as i128;
                self.push_extend(res.to_le_bytes());
            }
            OpConvertDoubleToUExtInt => {
                let num = f64::from_le_bytes(self.pop_qword());
                let res = num as u128;
                self.push_extend(res.to_le_bytes());
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
            OpBitNotExtInt => {
                let num = u128::from_le_bytes(self.pop_extend());
                let res = !num;
                self.push_extend(res.to_le_bytes());
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
            OpBitAndExtInt => {
                let num2 = u128::from_le_bytes(self.pop_extend());
                let num1 = u128::from_le_bytes(self.pop_extend());
                let res = num1 & num2;
                self.push_extend(res.to_le_bytes());
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
            OpBitOrExtInt => {
                let num2 = u128::from_le_bytes(self.pop_extend());
                let num1 = u128::from_le_bytes(self.pop_extend());
                let res = num1 | num2;
                self.push_extend(res.to_le_bytes());
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
            OpBitXorExtInt => {
                let num2 = u128::from_le_bytes(self.pop_extend());
                let num1 = u128::from_le_bytes(self.pop_extend());
                let res = num1 ^ num2;
                self.push_extend(res.to_le_bytes());
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
            OpICmpEqualExtInt => {
                let num2 = u128::from_le_bytes(self.pop_extend());
                let num1 = u128::from_le_bytes(self.pop_extend());
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
            OpICmpNotEqualExtInt => {
                let num2 = u128::from_le_bytes(self.pop_extend());
                let num1 = u128::from_le_bytes(self.pop_extend());
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
            OpICmpLessSExtInt => {
                let num2 = i128::from_le_bytes(self.pop_extend());
                let num1 = i128::from_le_bytes(self.pop_extend());
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
            OpICmpLessUExtInt => {
                let num2 = u128::from_le_bytes(self.pop_extend());
                let num1 = u128::from_le_bytes(self.pop_extend());
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
            OpICmpLessEqualSExtInt => {
                let num2 = i128::from_le_bytes(self.pop_extend());
                let num1 = i128::from_le_bytes(self.pop_extend());
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
            OpICmpLessEqualUExtInt => {
                let num2 = u128::from_le_bytes(self.pop_extend());
                let num1 = u128::from_le_bytes(self.pop_extend());
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
            OpICmpGreaterSExtInt => {
                let num2 = i128::from_le_bytes(self.pop_extend());
                let num1 = i128::from_le_bytes(self.pop_extend());
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
            OpICmpGreaterUExtInt => {
                let num2 = u128::from_le_bytes(self.pop_extend());
                let num1 = u128::from_le_bytes(self.pop_extend());
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
            OpICmpGreaterEqualSExtInt => {
                let num2 = i128::from_le_bytes(self.pop_extend());
                let num1 = i128::from_le_bytes(self.pop_extend());
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
            OpICmpGreaterEqualUExtInt => {
                let num2 = u128::from_le_bytes(self.pop_extend());
                let num1 = u128::from_le_bytes(self.pop_extend());
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
        }

        return Ok(());
    }
}

pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(msg: String) -> Self {
        Self { message: msg }
    }
}
