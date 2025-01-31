//! 反汇编模块

use std::fs::File;
use std::io::Read;
use crate::byte_handler::byte_reader::{read_byte, read_dword, read_extend, read_qword, read_word};
use crate::instr::Instruction;

/// 反汇编字节码文件
pub fn disassemble_file(path: &str) -> Result<(), String> {
    let mut file;
    match File::open(path) {
        Ok(temp) => file = temp,
        Err(err) => return Err(format!("Cannot open file '{}'! Error message: {}", path, err)),
    }
    let mut buffer = Vec::new();
    if let Err(err) = file.read_to_end(&mut buffer) {
        return Err(format!("Cannot read file '{}'! Error message: {}", path, err));
    }
    
    // 反汇编字节码
    disassemble_chunk("<main>", &buffer);
    
    return Ok(());
}

/// 反汇编代码块
fn disassemble_chunk(name: &str, chunk: &[u8]) {
    println!("====== Chunk {} ======", name);
    let mut offset = 0usize;  // 之后打印指令地址使用
    while offset < chunk.len() {
        let old_offset = offset;
        if let Ok((new_instr, new_offset)) = read_byte(chunk, offset) {  // 读取下一个字节码指令
            offset = new_offset;
            let instr_byte = u8::from_le_bytes(new_instr);
            if let Ok(instr) = Instruction::try_from(instr_byte) {
                match disassemble_instruction(instr, chunk, offset) {  // 反编译单条指令
                    Ok((result, new_offset)) => {
                        offset = new_offset;
                        println!("{:06X} | {}", old_offset, result);  // 打印指令
                    }
                    Err(err) => {
                        eprintln!("Disassemble Error: {}", err);
                        return;
                    }
                }
            } else {
                eprintln!("Disassemble Error: Invalid instruction '{:02X}'", instr_byte);
                return;
            }
        }
    }
    print!("======");
    for _i in 0..(name.len() + 8) {
        print!("=");
    }
    println!("======");
}

/// 反汇编指令
pub fn disassemble_instruction(instr: Instruction, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    use crate::instr::Instruction::*;
    match instr {
        OpReturn => Ok(simple("Return", "", chunk, offset)),
        OpLoadConstByte => const_byte("LoadConst", "Byte", chunk, offset),
        OpLoadConstWord => const_word("LoadConst", "Word", chunk, offset),
        OpLoadConstDword => const_dword("LoadConst", "Dword", chunk, offset),
        OpLoadConstQword => const_qword("LoadConst", "Qword", chunk, offset),
        OpLoadConstExtInt => const_extend("LoadConst", "ExtInt", chunk, offset),
        OpSignExtendByteToWord => Ok(simple("SignExtend", "Byte -> Word", chunk, offset)),
        OpSignExtendWordToDword => Ok(simple("SignExtend", "Word -> Dword", chunk, offset)),
        OpSignExtendDwordToQword => Ok(simple("SignExtend", "Dword -> Qword", chunk, offset)),
        OpZeroExtendByteToWord => Ok(simple("ZeroExtend", "Byte -> Word", chunk, offset)),
        OpZeroExtendWordToDword => Ok(simple("ZeroExtend", "Word -> Dword", chunk, offset)),
        OpZeroExtendDwordToQword => Ok(simple("ZeroExtend", "Dword -> Qword", chunk, offset)),
        OpTruncateQwordToDword => Ok(simple("Truncate", "Qword -> Dword", chunk, offset)),
        OpTruncateDwordToWord => Ok(simple("Truncate", "Dword -> Word", chunk, offset)),
        OpTruncateWordToByte => Ok(simple("Truncate", "Word -> Byte", chunk, offset)),
        OpIAddByte => Ok(simple("IAdd", "Byte", chunk, offset)),
        OpIAddWord => Ok(simple("IAdd", "Word", chunk, offset)),
        OpIAddDword => Ok(simple("IAdd", "Dword", chunk, offset)),
        OpIAddQword => Ok(simple("IAdd", "Qword", chunk, offset)),
        OpISubByte => Ok(simple("ISub", "Byte", chunk, offset)),
        OpISubWord => Ok(simple("ISub", "Word", chunk, offset)),
        OpISubDword => Ok(simple("ISub", "Dword", chunk, offset)),
        OpISubQword => Ok(simple("ISub", "Qword", chunk, offset)),
        OpIMulByte => Ok(simple("IMul", "Byte", chunk, offset)),
        OpIMulWord => Ok(simple("IMul", "Word", chunk, offset)),
        OpIMulDword => Ok(simple("IMul", "Dword", chunk, offset)),
        OpIMulQword => Ok(simple("IMul", "Qword", chunk, offset)),
        OpIDivSByte => Ok(simple("IDiv", "Signed Byte", chunk, offset)),
        OpIDivSWord => Ok(simple("IDiv", "Signed Word", chunk, offset)),
        OpIDivSDword => Ok(simple("IDiv", "Signed Dword", chunk, offset)),
        OpIDivSQword => Ok(simple("IDiv", "Signed Qword", chunk, offset)),
        OpIDivUByte => Ok(simple("IDiv", "Unsigned Byte", chunk, offset)),
        OpIDivUWord => Ok(simple("IDiv", "Unsigned Word", chunk, offset)),
        OpIDivUDword => Ok(simple("IDiv", "Unsigned Dword", chunk, offset)),
        OpIDivUQword => Ok(simple("IDiv", "Unsigned Qword", chunk, offset)),
        OpIModSByte => Ok(simple("IMod", "Signed Byte", chunk, offset)),
        OpIModSWord => Ok(simple("IMod", "Signed Word", chunk, offset)),
        OpIModSDword => Ok(simple("IMod", "Signed Dword", chunk, offset)),
        OpIModSQword => Ok(simple("IMod", "Signed Qword", chunk, offset)),
        OpIModUByte => Ok(simple("IMod", "Unsigned Byte", chunk, offset)),
        OpIModUWord => Ok(simple("IMod", "Unsigned Word", chunk, offset)),
        OpIModUDword => Ok(simple("IMod", "Unsigned Dword", chunk, offset)),
        OpIModUQword => Ok(simple("IMod", "Unsigned Qword", chunk, offset)),
        OpINegByte => Ok(simple("INeg", "Byte", chunk, offset)),
        OpINegWord => Ok(simple("INeg", "Word", chunk, offset)),
        OpINegDword => Ok(simple("INeg", "Dword", chunk, offset)),
        OpINegQword => Ok(simple("INeg", "Qword", chunk, offset)),
        OpSignExtendToExtInt => Ok(simple("SignExtend", "Qword -> ExtInt", chunk, offset)),
        OpZeroExtendToExtInt => Ok(simple("ZeroExtend", "Qword -> ExtInt", chunk, offset)),
        OpTruncateFromExtInt => Ok(simple("Truncate", "ExtInt -> Qword", chunk, offset)),
        OpIAddExtInt => Ok(simple("IAdd", "ExtInt", chunk, offset)),
        OpISubExtInt => Ok(simple("ISub", "ExtInt", chunk, offset)),
        OpIMulExtInt => Ok(simple("IMul", "ExtInt", chunk, offset)),
        OpIDivSExtInt => Ok(simple("IDiv", "Signed ExtInt", chunk, offset)),
        OpIDivUExtInt => Ok(simple("IDiv", "Unsigned ExtInt", chunk, offset)),
        OpIModSExtInt => Ok(simple("IMod", "Signed ExtInt", chunk, offset)),
        OpIModUExtInt => Ok(simple("IMod", "Unsigned ExtInt", chunk, offset)),
        OpINegExtInt => Ok(simple("INeg", "ExtInt", chunk, offset)),
        OpConvertSWordToFloat => Ok(simple("Convert", "Signed Word -> Float", chunk, offset)),
        OpConvertUWordToFloat => Ok(simple("Convert", "Unsigned Word -> Float", chunk, offset)),
        OpConvertSQwordToFloat => Ok(simple("Convert", "Signed Qword -> Float", chunk, offset)),
        OpConvertUQwordToFloat => Ok(simple("Convert", "Unsigned Qword -> Float", chunk, offset)),
        OpConvertSWordToDouble => Ok(simple("Convert", "Signed Word -> Double", chunk, offset)),
        OpConvertUWordToDouble => Ok(simple("Convert", "Unsigned Word -> Double", chunk, offset)),
        OpConvertSQwordToDouble => Ok(simple("Convert", "Signed Qword -> Double", chunk, offset)),
        OpConvertUQwordToDouble => Ok(simple("Convert", "Unsigned Qword -> Double", chunk, offset)),
        OpConvertFloatToSWord => Ok(simple("Convert", "Float -> Signed Word", chunk, offset)),
        OpConvertFloatToUWord => Ok(simple("Convert", "Float -> Unsigned Word", chunk, offset)),
        OpConvertFloatToSQword => Ok(simple("Convert", "Float -> Signed Qword", chunk, offset)),
        OpConvertFloatToUQword => Ok(simple("Convert", "Float -> Unsigned Qword", chunk, offset)),
        OpConvertDoubleToSWord => Ok(simple("Convert", "Double -> Signed Word", chunk, offset)),
        OpConvertDoubleToUWord => Ok(simple("Convert", "Double -> Unsigned Word", chunk, offset)),
        OpConvertDoubleToSQword => Ok(simple("Convert", "Double -> Signed Qword", chunk, offset)),
        OpConvertDoubleToUQword => Ok(simple("Convert", "Double -> Unsigned Qword", chunk, offset)),
        OpConvertSExtIntToFloat => Ok(simple("Convert", "Signed ExtInt -> Float", chunk, offset)),
        OpConvertUExtIntToFloat => Ok(simple("Convert", "Unsigned ExtInt -> Float", chunk, offset)),
        OpConvertSExtIntToDouble => Ok(simple("Convert", "Signed ExtInt -> Double", chunk, offset)),
        OpConvertUExtIntToDouble => Ok(simple("Convert", "Unsigned ExtInt -> Double", chunk, offset)),
        OpConvertFloatToSExtInt => Ok(simple("Convert", "Float -> Signed ExtInt", chunk, offset)),
        OpConvertFloatToUExtInt => Ok(simple("Convert", "Float -> Unsigned ExtInt", chunk, offset)),
        OpConvertDoubleToSExtInt => Ok(simple("Convert", "Double -> Signed ExtInt", chunk, offset)),
        OpConvertDoubleToUExtInt => Ok(simple("Convert", "Double -> Unsigned ExtInt", chunk, offset)),
        OpConvertFloatToDouble => Ok(simple("Convert", "Float -> Double", chunk, offset)),
        OpConvertDoubleToFloat => Ok(simple("Convert", "Double -> Float", chunk, offset)),
        OpFAddFloat => Ok(simple("FAdd", "Float", chunk, offset)),
        OpFAddDouble => Ok(simple("FAdd", "Double", chunk, offset)),
        OpFSubFloat => Ok(simple("FSub", "Float", chunk, offset)),
        OpFSubDouble => Ok(simple("FSub", "Double", chunk, offset)),
        OpFMulFloat => Ok(simple("FMul", "Float", chunk, offset)),
        OpFMulDouble => Ok(simple("FMul", "Double", chunk, offset)),
        OpFDivFloat => Ok(simple("FDiv", "Float", chunk, offset)),
        OpFDivDouble => Ok(simple("FDiv", "Double", chunk, offset)),
        OpFNegFloat => Ok(simple("FNeg", "Float", chunk, offset)),
        OpFNegDouble => Ok(simple("FNeg", "Double", chunk, offset)),
    }
}

/// 简单指令
#[inline]
fn simple(instr: &str, info: &str, _chunk: &[u8], offset: usize) -> (String, usize) {
    (format!("{:<20} [{:^25}]", instr, info), offset)
}

/// 加载常数字节指令
#[inline]
fn const_byte(instr: &str, info: &str, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    if let Ok((res_byte, res_offset)) = read_byte(chunk, offset) {
        let byte = u8::from_le_bytes(res_byte);
        let u_num = byte;
        let s_num = byte as i8;
        Ok((format!("{:<20} [{:^25}] {:02X} ({} or {})", instr, info, byte, u_num, s_num), res_offset))
    } else {
        Err("Not enough bytes to read: need 1 byte.".to_string())
    }
}

/// 加载常数单字指令
#[inline]
fn const_word(instr: &str, info: &str, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    if let Ok((res_word, res_offset)) = read_word(chunk, offset) {
        let word = u16::from_le_bytes(res_word);
        let u_num = word;
        let s_num = word as i16;
        Ok((format!("{:<20} [{:^25}] {:04X} ({} or {})", instr, info, word, u_num, s_num), res_offset))
    } else {
        Err("Not enough bytes to read: need 2 bytes.".to_string())
    }
}

/// 加载常数双字指令
#[inline]
fn const_dword(instr: &str, info: &str, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    if let Ok((res_dword, res_offset)) = read_dword(chunk, offset) {
        let dword = u32::from_le_bytes(res_dword);
        let u_num = dword;
        let s_num = dword as i32;
        let float = f32::from_le_bytes(res_dword);
        Ok((format!("{:<20} [{:^25}] {:08X} ({} or {} or {:e})", instr, info, dword, u_num, s_num, float), res_offset))
    } else {
        Err("Not enough bytes to read: need 4 bytes.".to_string())
    }
}

/// 加载常数四字指令
#[inline]
fn const_qword(instr: &str, info: &str, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    if let Ok((res_qword, res_offset)) = read_qword(chunk, offset) {
        let qword = u64::from_le_bytes(res_qword);
        let u_num = qword;
        let s_num = qword as i64;
        let double = f64::from_le_bytes(res_qword);
        Ok((format!("{:<20} [{:^25}] {:016X} ({} or {} or {:e})", instr, info, qword, u_num, s_num, double), res_offset))
    } else {
        Err("Not enough bytes to read: need 8 bytes.".to_string())
    }
}

/// 加载常数扩展整数指令
#[inline]
fn const_extend(instr: &str, info: &str, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    if let Ok((res_extend, res_offset)) = read_extend(chunk, offset) {
        let extend = u128::from_le_bytes(res_extend);
        let u_num = extend;
        let s_num = extend as i128;
        Ok((format!("{:<20} [{:^25}] {:032X} ({} or {})", instr, info, extend, u_num, s_num), res_offset))
    } else {
        Err("Not enough bytes to read: need 16 bytes.".to_string())
    }
}
