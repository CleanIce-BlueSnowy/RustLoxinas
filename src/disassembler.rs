//! 反汇编模块

use std::fs::File;
use std::{io, process};
use std::io::{Read, Write};
use crate::byte_handler::byte_reader::{read_byte, read_dword, read_oword, read_qword, read_word};
use crate::instr::{Instruction, SpecialFunction};

/// 反汇编字节码文件
pub fn disassemble_file(path: &str, output_file: &mut dyn Write) -> Result<(), String> {
    let mut file;
    match File::open(path) {
        Ok(temp) => file = temp,
        Err(err) => return Err(format!("Cannot open file '{}'! Error message: {}", path, err)),
    }
    
    let mut buffer = vec![];
    if let Err(err) = file.read_to_end(&mut buffer) {
        return Err(format!("Cannot read file '{}'! Error message: {}", path, err));
    }
    
    // 反汇编字节码
    if let Err(err) = disassemble_chunk("<main>", &buffer, output_file) {
        eprintln!("Cannot write to file: {}", err);
        process::exit(1);
    }
    
    return Ok(());
}

/// 反汇编代码块
fn disassemble_chunk(name: &str, chunk: &[u8], output_file: &mut dyn Write) -> io::Result<()> {
    writeln!(output_file, "====== Chunk {} ======", name)?;
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
                        writeln!(output_file, "{:08X} | {}", old_offset, result)?;  // 打印指令
                    }
                    Err(err) => {
                        eprintln!("Disassemble Error: {}", err);
                        return Ok(());
                    }
                }
            } else {
                eprintln!("Disassemble Error: Invalid instruction '{:02X}'", instr_byte);
                return Ok(());
            }
        }
    }
    
    write!(output_file, "======")?;
    for _i in 0..(name.len() + 8) {
        write!(output_file, "=")?;
    }
    writeln!(output_file, "======")?;
    
    return Ok(());
}

/// 反汇编指令
pub fn disassemble_instruction(instr: Instruction, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    use crate::instr::Instruction::*;
    
    match instr {
        OpSpecialFunction => special_function("SpecialFunction", chunk, offset),
        OpReturn => Ok(simple("Return", "", chunk, offset)),
        OpStackExtend => with_dword("StackExtend", "", chunk, offset),
        OpStackShrink => with_dword("StackShrink", "", chunk, offset),
        OpJump => jump("Jump", "", chunk, offset),
        OpJumpTrue => jump("Jump", "True", chunk, offset),
        OpJumpTruePop => jump("Jump", "True & Pop", chunk, offset),
        OpJumpFalse => jump("Jump", "False", chunk, offset),
        OpJumpFalsePop => jump("Jump", "False & Pop", chunk, offset),
        OpSignExtendByteToWord => Ok(simple("SignExtend", "Byte -> Word", chunk, offset)),
        OpSignExtendWordToDword => Ok(simple("SignExtend", "Word -> Dword", chunk, offset)),
        OpSignExtendDwordToQword => Ok(simple("SignExtend", "Dword -> Qword", chunk, offset)),
        OpSignExtendQwordToOword => Ok(simple("SignExtend", "Qword -> Oword", chunk, offset)),
        OpZeroExtendByteToWord => Ok(simple("ZeroExtend", "Byte -> Word", chunk, offset)),
        OpZeroExtendWordToDword => Ok(simple("ZeroExtend", "Word -> Dword", chunk, offset)),
        OpZeroExtendDwordToQword => Ok(simple("ZeroExtend", "Dword -> Qword", chunk, offset)),
        OpZeroExtendQwordToOword => Ok(simple("ZeroExtend", "Qword -> Oword", chunk, offset)),
        OpTruncateOwordToQword => Ok(simple("Truncate", "Oword -> Qword", chunk, offset)),
        OpTruncateQwordToDword => Ok(simple("Truncate", "Qword -> Dword", chunk, offset)),
        OpTruncateDwordToWord => Ok(simple("Truncate", "Dword -> Word", chunk, offset)),
        OpTruncateWordToByte => Ok(simple("Truncate", "Word -> Byte", chunk, offset)),
        OpIAddByte => Ok(simple("IAdd", "Byte", chunk, offset)),
        OpIAddWord => Ok(simple("IAdd", "Word", chunk, offset)),
        OpIAddDword => Ok(simple("IAdd", "Dword", chunk, offset)),
        OpIAddQword => Ok(simple("IAdd", "Qword", chunk, offset)),
        OpIAddOword => Ok(simple("IAdd", "Oword", chunk, offset)),
        OpISubByte => Ok(simple("ISub", "Byte", chunk, offset)),
        OpISubWord => Ok(simple("ISub", "Word", chunk, offset)),
        OpISubDword => Ok(simple("ISub", "Dword", chunk, offset)),
        OpISubQword => Ok(simple("ISub", "Qword", chunk, offset)),
        OpISubOword => Ok(simple("ISub", "Oword", chunk, offset)),
        OpIMulByte => Ok(simple("IMul", "Byte", chunk, offset)),
        OpIMulWord => Ok(simple("IMul", "Word", chunk, offset)),
        OpIMulDword => Ok(simple("IMul", "Dword", chunk, offset)),
        OpIMulQword => Ok(simple("IMul", "Qword", chunk, offset)),
        OpIMulOword => Ok(simple("IMul", "Oword", chunk, offset)),
        OpIDivSByte => Ok(simple("IDiv", "Signed Byte", chunk, offset)),
        OpIDivSWord => Ok(simple("IDiv", "Signed Word", chunk, offset)),
        OpIDivSDword => Ok(simple("IDiv", "Signed Dword", chunk, offset)),
        OpIDivSQword => Ok(simple("IDiv", "Signed Qword", chunk, offset)),
        OpIDivSOword => Ok(simple("IDiv", "Signed Oword", chunk, offset)),
        OpIDivUByte => Ok(simple("IDiv", "Unsigned Byte", chunk, offset)),
        OpIDivUWord => Ok(simple("IDiv", "Unsigned Word", chunk, offset)),
        OpIDivUDword => Ok(simple("IDiv", "Unsigned Dword", chunk, offset)),
        OpIDivUQword => Ok(simple("IDiv", "Unsigned Qword", chunk, offset)),
        OpIDivUOword => Ok(simple("IDiv", "Unsigned Oword", chunk, offset)),
        OpIModSByte => Ok(simple("IMod", "Signed Byte", chunk, offset)),
        OpIModSWord => Ok(simple("IMod", "Signed Word", chunk, offset)),
        OpIModSDword => Ok(simple("IMod", "Signed Dword", chunk, offset)),
        OpIModSQword => Ok(simple("IMod", "Signed Qword", chunk, offset)),
        OpIModSOword => Ok(simple("IMod", "Signed Oword", chunk, offset)),
        OpIModUByte => Ok(simple("IMod", "Unsigned Byte", chunk, offset)),
        OpIModUWord => Ok(simple("IMod", "Unsigned Word", chunk, offset)),
        OpIModUDword => Ok(simple("IMod", "Unsigned Dword", chunk, offset)),
        OpIModUQword => Ok(simple("IMod", "Unsigned Qword", chunk, offset)),
        OpIModUOword => Ok(simple("IMod", "Unsigned Oword", chunk, offset)),
        OpINegByte => Ok(simple("INeg", "Byte", chunk, offset)),
        OpINegWord => Ok(simple("INeg", "Word", chunk, offset)),
        OpINegDword => Ok(simple("INeg", "Dword", chunk, offset)),
        OpINegQword => Ok(simple("INeg", "Qword", chunk, offset)),
        OpINegOword => Ok(simple("INeg", "Oword", chunk, offset)),
        OpConvertSWordToFloat => Ok(simple("Convert", "Signed Word -> Float", chunk, offset)),
        OpConvertUWordToFloat => Ok(simple("Convert", "Unsigned Word -> Float", chunk, offset)),
        OpConvertSQwordToFloat => Ok(simple("Convert", "Signed Qword -> Float", chunk, offset)),
        OpConvertUQwordToFloat => Ok(simple("Convert", "Unsigned Qword -> Float", chunk, offset)),
        OpConvertSOwordToFloat => Ok(simple("Convert", "Signed Oword -> Float", chunk, offset)),
        OpConvertUOwordToFloat => Ok(simple("Convert", "Unsigned Oword -> Float", chunk, offset)),
        OpConvertSWordToDouble => Ok(simple("Convert", "Signed Word -> Double", chunk, offset)),
        OpConvertUWordToDouble => Ok(simple("Convert", "Unsigned Word -> Double", chunk, offset)),
        OpConvertSQwordToDouble => Ok(simple("Convert", "Signed Qword -> Double", chunk, offset)),
        OpConvertUQwordToDouble => Ok(simple("Convert", "Unsigned Qword -> Double", chunk, offset)),
        OpConvertSOwordToDouble => Ok(simple("Convert", "Signed Oword -> Double", chunk, offset)),
        OpConvertUOwordToDouble => Ok(simple("Convert", "Unsigned Oword -> Double", chunk, offset)),
        OpConvertFloatToSWord => Ok(simple("Convert", "Float -> Signed Word", chunk, offset)),
        OpConvertFloatToUWord => Ok(simple("Convert", "Float -> Unsigned Word", chunk, offset)),
        OpConvertFloatToSQword => Ok(simple("Convert", "Float -> Signed Qword", chunk, offset)),
        OpConvertFloatToUQword => Ok(simple("Convert", "Float -> Unsigned Qword", chunk, offset)),
        OpConvertFloatToSOword => Ok(simple("Convert", "Float -> Signed Oword", chunk, offset)),
        OpConvertFloatToUOword => Ok(simple("Convert", "Float -> Unsigned Oword", chunk, offset)),
        OpConvertDoubleToSWord => Ok(simple("Convert", "Double -> Signed Word", chunk, offset)),
        OpConvertDoubleToUWord => Ok(simple("Convert", "Double -> Unsigned Word", chunk, offset)),
        OpConvertDoubleToSQword => Ok(simple("Convert", "Double -> Signed Qword", chunk, offset)),
        OpConvertDoubleToUQword => Ok(simple("Convert", "Double -> Unsigned Qword", chunk, offset)),
        OpConvertDoubleToSOword => Ok(simple("Convert", "Double -> Signed Oword", chunk, offset)),
        OpConvertDoubleToUOword => Ok(simple("Convert", "Double -> Unsigned Oword", chunk, offset)),
        OpConvertFloatToDouble => Ok(simple("Convert", "Float -> Double", chunk, offset)),
        OpConvertDoubleToFloat => Ok(simple("Convert", "Double -> Float", chunk, offset)),
        OpConvertByteToBool => Ok(simple("Convert", "Byte -> Bool", chunk, offset)),
        OpConvertWordToBool => Ok(simple("Convert", "Word -> Bool", chunk, offset)),
        OpConvertDwordToBool => Ok(simple("Convert", "Dword -> Bool", chunk, offset)),
        OpConvertQwordToBool => Ok(simple("Convert", "Qword -> Bool", chunk, offset)),
        OpConvertOwordToBool => Ok(simple("Convert", "Oword -> Bool", chunk, offset)),
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
        OpBitNotByte => Ok(simple("BitNot", "Byte", chunk, offset)),
        OpBitNotWord => Ok(simple("BitNot", "Word", chunk, offset)),
        OpBitNotDword => Ok(simple("BitNot", "Dword", chunk, offset)),
        OpBitNotQword => Ok(simple("BitNot", "Qword", chunk, offset)),
        OpBitNotOword => Ok(simple("BitNot", "Oword", chunk, offset)),
        OpBitAndByte => Ok(simple("BitAnd", "Byte", chunk, offset)),
        OpBitAndWord => Ok(simple("BitAnd", "Word", chunk, offset)),
        OpBitAndDword => Ok(simple("BitAnd", "Dword", chunk, offset)),
        OpBitAndQword => Ok(simple("BitAnd", "Qword", chunk, offset)),
        OpBitAndOword => Ok(simple("BitAnd", "Oword", chunk, offset)),
        OpBitOrByte => Ok(simple("BitOr", "Byte", chunk, offset)),
        OpBitOrWord => Ok(simple("BitOr", "Word", chunk, offset)),
        OpBitOrDword => Ok(simple("BitOr", "Dword", chunk, offset)),
        OpBitOrQword => Ok(simple("BitOr", "Qword", chunk, offset)),
        OpBitOrOword => Ok(simple("BitOr", "Oword", chunk, offset)),
        OpBitXorByte => Ok(simple("BitXor", "Byte", chunk, offset)),
        OpBitXorWord => Ok(simple("BitXor", "Word", chunk, offset)),
        OpBitXorDword => Ok(simple("BitXor", "Dword", chunk, offset)),
        OpBitXorQword => Ok(simple("BitXor", "Qword", chunk, offset)),
        OpBitXorOword => Ok(simple("BitXor", "Oword", chunk, offset)),
        OpShiftLeftByte => Ok(simple("ShiftLeft", "Byte", chunk, offset)),
        OpShiftLeftWord => Ok(simple("ShiftLeft", "Word", chunk, offset)),
        OpShiftLeftDword => Ok(simple("ShiftLeft", "Dword", chunk, offset)),
        OpShiftLeftQword => Ok(simple("ShiftLeft", "Qword", chunk, offset)),
        OpShiftLeftOword => Ok(simple("ShiftLeft", "Oword", chunk, offset)),
        OpSignShiftRightByte => Ok(simple("SignShiftRight", "Byte", chunk, offset)),
        OpSignShiftRightWord => Ok(simple("SignShiftRight", "Word", chunk, offset)),
        OpSignShiftRightDword => Ok(simple("SignShiftRight", "Dword", chunk, offset)),
        OpSignShiftRightQword => Ok(simple("SignShiftRight", "Qword", chunk, offset)),
        OpSignShiftRightOword => Ok(simple("SignShiftRight", "Oword", chunk, offset)),
        OpZeroShiftRightByte => Ok(simple("ZeroShiftRight", "Byte", chunk, offset)),
        OpZeroShiftRightWord => Ok(simple("ZeroShiftRight", "Word", chunk, offset)),
        OpZeroShiftRightDword => Ok(simple("ZeroShiftRight", "Dword", chunk, offset)),
        OpZeroShiftRightQword => Ok(simple("ZeroShiftRight", "Qword", chunk, offset)),
        OpZeroShiftRightOword => Ok(simple("ZeroShiftRight", "Oword", chunk, offset)),
        OpICmpEqualByte => Ok(simple("ICmpEqual", "Byte", chunk, offset)),
        OpICmpEqualWord => Ok(simple("ICmpEqual", "Word", chunk, offset)),
        OpICmpEqualDword => Ok(simple("ICmpEqual", "Dword", chunk, offset)),
        OpICmpEqualQword => Ok(simple("ICmpEqual", "Qword", chunk, offset)),
        OpICmpEqualOword => Ok(simple("ICmpEqual", "Oword", chunk, offset)),
        OpICmpNotEqualByte => Ok(simple("ICmpNotEqual", "Byte", chunk, offset)),
        OpICmpNotEqualWord => Ok(simple("ICmpNotEqual", "Word", chunk, offset)),
        OpICmpNotEqualDword => Ok(simple("ICmpNotEqual", "Dword", chunk, offset)),
        OpICmpNotEqualQword => Ok(simple("ICmpNotEqual", "Qword", chunk, offset)),
        OpICmpNotEqualOword => Ok(simple("ICmpNotEqual", "Oword", chunk, offset)),
        OpICmpLessSByte => Ok(simple("ICmpLess", "Signed Byte", chunk, offset)),
        OpICmpLessSWord => Ok(simple("ICmpLess", "Signed Word", chunk, offset)),
        OpICmpLessSDword => Ok(simple("ICmpLess", "Signed Dword", chunk, offset)),
        OpICmpLessSQword => Ok(simple("ICmpLess", "Signed Qword", chunk, offset)),
        OpICmpLessSOword => Ok(simple("ICmpLess", "Signed Oword", chunk, offset)),
        OpICmpLessUByte => Ok(simple("ICmpLess", "Unsigned Byte", chunk, offset)),
        OpICmpLessUWord => Ok(simple("ICmpLess", "Unsigned Word", chunk, offset)),
        OpICmpLessUDword => Ok(simple("ICmpLess", "Unsigned Dword", chunk, offset)),
        OpICmpLessUQword => Ok(simple("ICmpLess", "Unsigned Qword", chunk, offset)),
        OpICmpLessUOword => Ok(simple("ICmpLess", "Unsigned Oword", chunk, offset)),
        OpICmpLessEqualSByte => Ok(simple("ICmpLessEqual", "Signed Byte", chunk, offset)),
        OpICmpLessEqualSWord => Ok(simple("ICmpLessEqual", "Signed Word", chunk, offset)),
        OpICmpLessEqualSDword => Ok(simple("ICmpLessEqual", "Signed Dword", chunk, offset)),
        OpICmpLessEqualSQword => Ok(simple("ICmpLessEqual", "Signed Qword", chunk, offset)),
        OpICmpLessEqualSOword => Ok(simple("ICmpLessEqual", "Signed Oword", chunk, offset)),
        OpICmpLessEqualUByte => Ok(simple("ICmpLessEqual", "Unsigned Byte", chunk, offset)),
        OpICmpLessEqualUWord => Ok(simple("ICmpLessEqual", "Unsigned Word", chunk, offset)),
        OpICmpLessEqualUDword => Ok(simple("ICmpLessEqual", "Unsigned Dword", chunk, offset)),
        OpICmpLessEqualUQword => Ok(simple("ICmpLessEqual", "Unsigned Qword", chunk, offset)),
        OpICmpLessEqualUOword => Ok(simple("ICmpLessEqual", "Unsigned Oword", chunk, offset)),
        OpICmpGreaterSByte => Ok(simple("ICmpGreater", "Signed Byte", chunk, offset)),
        OpICmpGreaterSWord => Ok(simple("ICmpGreater", "Signed Word", chunk, offset)),
        OpICmpGreaterSDword => Ok(simple("ICmpGreater", "Signed Dword", chunk, offset)),
        OpICmpGreaterSQword => Ok(simple("ICmpGreater", "Signed Qword", chunk, offset)),
        OpICmpGreaterSOword => Ok(simple("ICmpGreater", "Signed Oword", chunk, offset)),
        OpICmpGreaterUByte => Ok(simple("ICmpGreater", "Unsigned Byte", chunk, offset)),
        OpICmpGreaterUWord => Ok(simple("ICmpGreater", "Unsigned Word", chunk, offset)),
        OpICmpGreaterUDword => Ok(simple("ICmpGreater", "Unsigned Dword", chunk, offset)),
        OpICmpGreaterUQword => Ok(simple("ICmpGreater", "Unsigned Qword", chunk, offset)),
        OpICmpGreaterUOword => Ok(simple("ICmpGreater", "Unsigned Oword", chunk, offset)),
        OpICmpGreaterEqualSByte => Ok(simple("ICmpGreaterEqual", "Signed Byte", chunk, offset)),
        OpICmpGreaterEqualSWord => Ok(simple("ICmpGreaterEqual", "Signed Word", chunk, offset)),
        OpICmpGreaterEqualSDword => Ok(simple("ICmpGreaterEqual", "Signed Dword", chunk, offset)),
        OpICmpGreaterEqualSQword => Ok(simple("ICmpGreaterEqual", "Signed Qword", chunk, offset)),
        OpICmpGreaterEqualSOword => Ok(simple("ICmpGreaterEqual", "Signed Oword", chunk, offset)),
        OpICmpGreaterEqualUByte => Ok(simple("ICmpGreaterEqual", "Unsigned Byte", chunk, offset)),
        OpICmpGreaterEqualUWord => Ok(simple("ICmpGreaterEqual", "Unsigned Word", chunk, offset)),
        OpICmpGreaterEqualUDword => Ok(simple("ICmpGreaterEqual", "Unsigned Dword", chunk, offset)),
        OpICmpGreaterEqualUQword => Ok(simple("ICmpGreaterEqual", "Unsigned Qword", chunk, offset)),
        OpICmpGreaterEqualUOword => Ok(simple("ICmpGreaterEqual", "Unsigned Oword", chunk, offset)),
        OpFCmpEqualFloat => Ok(simple("FCmpEqual", "Float", chunk, offset)),
        OpFCmpEqualDouble => Ok(simple("FCmpEqual", "Double", chunk, offset)),
        OpFCmpNotEqualFloat => Ok(simple("FCmpNotEqual", "Float", chunk, offset)),
        OpFCmpNotEqualDouble => Ok(simple("FCmpNotEqual", "Double", chunk, offset)),
        OpFCmpLessFloat => Ok(simple("FCmpLess", "Float", chunk, offset)),
        OpFCmpLessDouble => Ok(simple("FCmpLess", "Double", chunk, offset)),
        OpFCmpLessEqualFloat => Ok(simple("FCmpLessEqual", "Float", chunk, offset)),
        OpFCmpLessEqualDouble => Ok(simple("FCmpLessEqual", "Double", chunk, offset)),
        OpFCmpGreaterFloat => Ok(simple("FCmpGreater", "Float", chunk, offset)),
        OpFCmpGreaterDouble => Ok(simple("FCmpGreater", "Double", chunk, offset)),
        OpFCmpGreaterEqualFloat => Ok(simple("FCmpGreaterEqual", "Float", chunk, offset)),
        OpFCmpGreaterEqualDouble => Ok(simple("FCmpGreaterEqual", "Double", chunk, offset)),
        OpPopByte => Ok(simple("Pop", "Byte", chunk, offset)),
        OpPopWord => Ok(simple("Pop", "Word", chunk, offset)),
        OpPopDword => Ok(simple("Pop", "Dword", chunk, offset)),
        OpPopQword => Ok(simple("Pop", "Qword", chunk, offset)),
        OpPopOword => Ok(simple("Pop", "Oword", chunk, offset)),
        OpPushByte => const_byte("Push", "Byte", chunk, offset),
        OpPushWord => const_word("Push", "Word", chunk, offset),
        OpPushDword => const_dword("Push", "Dword", chunk, offset),
        OpPushQword => const_qword("Push", "Qword", chunk, offset),
        OpPushOword => const_oword("Push", "Oword", chunk, offset),
        OpCopyByte => Ok(simple("Copy", "Byte", chunk, offset)),
        OpCopyWord => Ok(simple("Copy", "Word", chunk, offset)),
        OpCopyDword => Ok(simple("Copy", "Dword", chunk, offset)),
        OpCopyQword => Ok(simple("Copy", "Qword", chunk, offset)),
        OpCopyOword => Ok(simple("Copy", "Oword", chunk, offset)),
        OpGetLocalByte => with_dword("GetLocal", "Byte", chunk, offset),
        OpGetLocalWord => with_dword("GetLocal", "Word", chunk, offset),
        OpGetLocalDword => with_dword("GetLocal", "Dword", chunk, offset),
        OpGetLocalQword => with_dword("GetLocal", "Qword", chunk, offset),
        OpGetLocalOword => with_dword("GetLocal", "Oword", chunk, offset),
        OpSetLocalByte => with_dword("SetLocal", "Byte", chunk, offset),
        OpSetLocalWord => with_dword("SetLocal", "Word", chunk, offset),
        OpSetLocalDword => with_dword("SetLocal", "Dword", chunk, offset),
        OpSetLocalQword => with_dword("SetLocal", "Qword", chunk, offset),
        OpSetLocalOword => with_dword("SetLocal", "Oword", chunk, offset),
        OpGetReferenceByte => with_dword("GetReference", "Byte", chunk, offset),
        OpGetReferenceWord => with_dword("GetReference", "Word", chunk, offset),
        OpGetReferenceDword => with_dword("GetReference", "Dword", chunk, offset),
        OpGetReferenceQword => with_dword("GetReference", "Qword", chunk, offset),
        OpGetReferenceOword => with_dword("GetReference", "Oword", chunk, offset),
        OpSetReferenceByte => with_dword("SetReference", "Byte", chunk, offset),
        OpSetReferenceWord => with_dword("SetReference", "Word", chunk, offset),
        OpSetReferenceDword => with_dword("SetReference", "Dword", chunk, offset),
        OpSetReferenceQword => with_dword("SetReference", "Qword", chunk, offset),
        OpSetReferenceOword => with_dword("SetReference", "Oword", chunk, offset),
    }
}

/// 通用格式化字符串
macro_rules! fmt_str {
    () => {
        "{:<20} [{:^25}] {}"
    }
}

/// 简单指令
#[inline]
#[must_use]
fn simple(instr: &str, info: &str, _chunk: &[u8], offset: usize) -> (String, usize) {
    (format!(fmt_str!(), instr, info, ""), offset)
}

fn jump(instr: &str, info: &str, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    if let Ok((res_goto, res_offset)) = read_dword(chunk, offset) {
        let goto = i32::from_le_bytes(res_goto);
        let location = res_offset as isize + goto as isize;
        Ok((format!(fmt_str!(), instr, info, format!("{} (at {:08X})", goto, location)), res_offset))
    } else {
        Err("Not enough bytes to read: need 4 bytes.".to_string())
    }
}

/// 常数字节指令
fn const_byte(instr: &str, info: &str, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    if let Ok((res_byte, res_offset)) = read_byte(chunk, offset) {
        let byte = u8::from_le_bytes(res_byte);
        let u_num = byte;
        Ok((format!(fmt_str!(), instr, info, format!("{:02X} ({})", byte, u_num)), res_offset))
    } else {
        Err("Not enough bytes to read: need 1 byte.".to_string())
    }
}

/// 常数单字指令
fn const_word(instr: &str, info: &str, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    if let Ok((res_word, res_offset)) = read_word(chunk, offset) {
        let word = u16::from_le_bytes(res_word);
        let u_num = word;
        Ok((format!(fmt_str!(), instr, info, format!("{:04X} ({})", word, u_num)), res_offset))
    } else {
        Err("Not enough bytes to read: need 2 bytes.".to_string())
    }
}

/// 常数双字指令
fn const_dword(instr: &str, info: &str, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    if let Ok((res_dword, res_offset)) = read_dword(chunk, offset) {
        let dword = u32::from_le_bytes(res_dword);
        let u_num = dword;
        let float = f32::from_le_bytes(res_dword);
        Ok((format!(fmt_str!(), instr, info, format!("{:08X} ({} or {:e})", dword, u_num, float)), res_offset))
    } else {
        Err("Not enough bytes to read: need 4 bytes.".to_string())
    }
}

/// 常数四字指令
fn const_qword(instr: &str, info: &str, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    if let Ok((res_qword, res_offset)) = read_qword(chunk, offset) {
        let qword = u64::from_le_bytes(res_qword);
        let u_num = qword;
        let double = f64::from_le_bytes(res_qword);
        Ok((format!(fmt_str!(), instr, info, format!("{:016X} ({} or {:e})", qword, u_num, double)), res_offset))
    } else {
        Err("Not enough bytes to read: need 8 bytes.".to_string())
    }
}

/// 常数扩展整数指令
fn const_oword(instr: &str, info: &str, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    if let Ok((res_oword, res_offset)) = read_oword(chunk, offset) {
        let oword = u128::from_le_bytes(res_oword);
        let u_num = oword;
        Ok((format!(fmt_str!(), instr, info, format!("{:032X} ({})", oword, u_num)), res_offset))
    } else {
        Err("Not enough bytes to read: need 16 bytes.".to_string())
    }
}

/// 带有 4 字节参数的指令
fn with_dword(instr: &str, info: &str, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    if let Ok((res_slot, new_offset)) = read_dword(chunk, offset) {
        let slot = u32::from_le_bytes(res_slot);
        Ok((format!(fmt_str!(), instr, info, format!("{:08X}", slot)), new_offset))
    } else {
        Err("Not enough bytes to read: need 4 bytes.".to_string())
    }
}

/// 特殊功能指令
fn special_function(instr: &str, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    if let Ok((res_func, new_offset)) = read_byte(chunk, offset) {
        if let Ok(special_func) = SpecialFunction::try_from(u8::from_le_bytes(res_func)) {
            let result = parse_special_function(instr, special_func, chunk, new_offset)?;
            Ok(result)
        } else {
            Err(format!("Disassemble Error: Invalid special function instruction '{:02X}'", u8::from_le_bytes(res_func)))
        }
    } else {
        Err("Not enough bytes to read: need 1 bytes".to_string())
    }
}

/// 解析特殊功能
#[inline]
fn parse_special_function(instr: &str, special_func: SpecialFunction, chunk: &[u8], offset: usize) -> Result<(String, usize), String> {
    use crate::instr::SpecialFunction::*;
    
    match special_func {
        PrintByte => Ok(simple(instr, "Print Byte", chunk, offset)),
        PrintSByte => Ok(simple(instr, "Print SByte", chunk, offset)),
        PrintShort => Ok(simple(instr, "Print Short", chunk, offset)),
        PrintUShort => Ok(simple(instr, "Print UShort", chunk, offset)),
        PrintInt => Ok(simple(instr, "Print Int", chunk, offset)),
        PrintUInt => Ok(simple(instr, "Print UInt", chunk, offset)),
        PrintLong => Ok(simple(instr, "Print Long", chunk, offset)),
        PrintULong => Ok(simple(instr, "Print ULong", chunk, offset)),
        PrintExtInt => Ok(simple(instr, "Print ExtInt", chunk, offset)),
        PrintUExtInt => Ok(simple(instr, "Print UExtInt", chunk, offset)),
        PrintFloat => Ok(simple(instr, "Print Float", chunk, offset)),
        PrintDouble => Ok(simple(instr, "Print Double", chunk, offset)),
        PrintBool => Ok(simple(instr, "Print Bool", chunk, offset)),
        PrintChar => Ok(simple(instr, "Print Char", chunk, offset)),
        PrintNewLine => Ok(simple(instr, "Print NewLine", chunk, offset)),
    }
}
