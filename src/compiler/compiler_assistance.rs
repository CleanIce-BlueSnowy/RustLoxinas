//! 编译器——辅助功能模块

use std::collections::LinkedList;

use crate::byte_handler::byte_writer::{write_byte, write_dword, write_oword, write_qword, write_word};
use crate::compiler::Compiler;
use crate::instr::Instruction;
use crate::instr::Instruction::*;
use crate::types::{ValueFloatType, ValueType};

impl Compiler {
    /// 写入类型转换指令
    pub fn convert_types(&mut self, from: &ValueType, to: &ValueType) {
        use crate::types::ValueType::*;
        use crate::types::ValueIntegerType::*;
        use crate::types::ValueFloatType::*;
        match (from, to) {
            (Integer(from), Integer(to)) => {
                match (from, to) {
                    (Byte, Byte) | (Byte, SByte) | (SByte, Byte) | (SByte, SByte) => (),
                    (Byte, Short) | (Byte, UShort) => {
                        self.write_code(OpZeroExtendByteToWord);
                    }
                    (Byte, Int) | (Byte, UInt) => {
                        self.write_code(OpZeroExtendByteToWord);
                        self.write_code(OpZeroExtendWordToDword);
                    }
                    (Byte, Long) | (Byte, ULong) => {
                        self.write_code(OpZeroExtendByteToWord);
                        self.write_code(OpZeroExtendWordToDword);
                        self.write_code(OpZeroExtendDwordToQword);
                    }
                    (Byte, ExtInt) | (Byte, UExtInt) => {
                        self.write_code(OpZeroExtendByteToWord);
                        self.write_code(OpZeroExtendWordToDword);
                        self.write_code(OpZeroExtendDwordToQword);
                        self.write_code(OpZeroExtendQwordToOword);
                    }
                    (SByte, Short) | (SByte, UShort) => {
                        self.write_code(OpSignExtendByteToWord);
                    }
                    (SByte, Int) | (SByte, UInt) => {
                        self.write_code(OpSignExtendByteToWord);
                        self.write_code(OpSignExtendWordToDword);
                    }
                    (SByte, Long) | (SByte, ULong) => {
                        self.write_code(OpSignExtendByteToWord);
                        self.write_code(OpSignExtendWordToDword);
                        self.write_code(OpSignExtendDwordToQword);
                    }
                    (SByte, ExtInt) | (SByte, UExtInt) => {
                        self.write_code(OpSignExtendByteToWord);
                        self.write_code(OpSignExtendWordToDword);
                        self.write_code(OpSignExtendDwordToQword);
                        self.write_code(OpSignExtendQwordToOword);
                    }
                    (Short, Byte) | (Short, SByte) | (UShort, Byte) | (UShort, SByte) => {
                        self.write_code(OpTruncateWordToByte);
                    }
                    (Short, Short) | (Short, UShort) | (UShort, Short) | (UShort, UShort) => (),
                    (Short, Int) | (Short, UInt) => {
                        self.write_code(OpSignExtendWordToDword);
                    }
                    (Short, Long) | (Short, ULong) => {
                        self.write_code(OpSignExtendWordToDword);
                        self.write_code(OpSignExtendDwordToQword);
                    }
                    (Short, ExtInt) | (Short, UExtInt) => {
                        self.write_code(OpSignExtendWordToDword);
                        self.write_code(OpSignExtendDwordToQword);
                        self.write_code(OpSignExtendQwordToOword);
                    }
                    (UShort, Int) | (UShort, UInt) => {
                        self.write_code(OpZeroExtendWordToDword);
                    }
                    (UShort, Long) | (UShort, ULong) => {
                        self.write_code(OpZeroExtendWordToDword);
                        self.write_code(OpZeroExtendDwordToQword);
                    }
                    (UShort, ExtInt) | (UShort, UExtInt) => {
                        self.write_code(OpZeroExtendWordToDword);
                        self.write_code(OpZeroExtendDwordToQword);
                        self.write_code(OpZeroExtendQwordToOword);
                    }
                    (Int, Byte) | (Int, SByte) | (UInt, Byte) | (UInt, SByte) => {
                        self.write_code(OpTruncateDwordToWord);
                        self.write_code(OpTruncateWordToByte);
                    }
                    (Int, Short) | (Int, UShort) | (UInt, Short) | (UInt, UShort) => {
                        self.write_code(OpTruncateDwordToWord);
                    }
                    (Int, Int) | (Int, UInt) | (UInt, Int) | (UInt, UInt) => (),
                    (Int, Long) | (Int, ULong) => {
                        self.write_code(OpSignExtendDwordToQword);
                    }
                    (Int, ExtInt) | (Int, UExtInt) => {
                        self.write_code(OpSignExtendDwordToQword);
                        self.write_code(OpSignExtendQwordToOword);
                    }
                    (UInt, Long) | (UInt, ULong) => {
                        self.write_code(OpZeroExtendDwordToQword);
                    }
                    (UInt, ExtInt) | (UInt, UExtInt) => {
                        self.write_code(OpZeroExtendDwordToQword);
                        self.write_code(OpZeroExtendQwordToOword);
                    }
                    (Long, Byte) | (Long, SByte) | (ULong, Byte) | (ULong, SByte) => {
                        self.write_code(OpTruncateQwordToDword);
                        self.write_code(OpTruncateDwordToWord);
                        self.write_code(OpTruncateWordToByte);
                    }
                    (Long, Short) | (Long, UShort) | (ULong, Short) | (ULong, UShort) => {
                        self.write_code(OpTruncateQwordToDword);
                        self.write_code(OpTruncateDwordToWord);
                    }
                    (Long, Int) | (Long, UInt) | (ULong, Int) | (ULong, UInt) => {
                        self.write_code(OpTruncateQwordToDword);
                    }
                    (Long, Long) | (Long, ULong) | (ULong, Long) | (ULong, ULong) => (),
                    (Long, ExtInt) | (Long, UExtInt) => {
                        self.write_code(OpSignExtendQwordToOword);
                    }
                    (ULong, ExtInt) | (ULong, UExtInt) => {
                        self.write_code(OpZeroExtendQwordToOword);
                    }
                    (ExtInt, Byte) | (ExtInt, SByte) | (UExtInt, Byte) | (UExtInt, SByte) => {
                        self.write_code(OpTruncateOwordToQword);
                        self.write_code(OpTruncateQwordToDword);
                        self.write_code(OpTruncateDwordToWord);
                        self.write_code(OpTruncateWordToByte);
                    }
                    (ExtInt, Short) | (ExtInt, UShort) | (UExtInt, Short) | (UExtInt, UShort) => {
                        self.write_code(OpTruncateOwordToQword);
                        self.write_code(OpTruncateQwordToDword);
                        self.write_code(OpTruncateDwordToWord);
                    }
                    (ExtInt, Int) | (ExtInt, UInt) | (UExtInt, Int) | (UExtInt, UInt) => {
                        self.write_code(OpTruncateOwordToQword);
                        self.write_code(OpTruncateQwordToDword);
                    }
                    (ExtInt, Long) | (ExtInt, ULong) | (UExtInt, Long) | (UExtInt, ULong) => {
                        self.write_code(OpTruncateOwordToQword);
                    }
                    (ExtInt, ExtInt) | (ExtInt, UExtInt) | (UExtInt, ExtInt) | (UExtInt, UExtInt) => (),
                }
            }
            (Integer(from), ValueType::Float(to)) => {
                match (from, to) {
                    (Byte, ValueFloatType::Float) => {
                        self.write_code(OpZeroExtendByteToWord);
                        self.write_code(OpConvertUWordToFloat);
                    }
                    (SByte, ValueFloatType::Float) => {
                        self.write_code(OpSignExtendByteToWord);
                        self.write_code(OpConvertSWordToFloat);
                    }
                    (Short, ValueFloatType::Float) => {
                        self.write_code(OpConvertSWordToFloat);
                    }
                    (UShort, ValueFloatType::Float) => {
                        self.write_code(OpConvertUWordToFloat);
                    }
                    (Int, ValueFloatType::Float) => {
                        self.write_code(OpSignExtendDwordToQword);
                        self.write_code(OpConvertSQwordToFloat);
                    }
                    (UInt, ValueFloatType::Float) => {
                        self.write_code(OpZeroExtendDwordToQword);
                        self.write_code(OpConvertUQwordToFloat);
                    }
                    (Long, ValueFloatType::Float) => {
                        self.write_code(OpConvertSQwordToFloat);
                    }
                    (ULong, ValueFloatType::Float) => {
                        self.write_code(OpConvertUQwordToFloat);
                    }
                    (ExtInt, ValueFloatType::Float) => {
                        self.write_code(OpConvertSOwordToFloat);
                    }
                    (UExtInt, ValueFloatType::Float) => {
                        self.write_code(OpConvertUOwordToFloat);
                    }
                    (Byte, Double) => {
                        self.write_code(OpZeroExtendByteToWord);
                        self.write_code(OpConvertUWordToDouble);
                    }
                    (SByte, Double) => {
                        self.write_code(OpSignExtendByteToWord);
                        self.write_code(OpConvertSWordToDouble);
                    }
                    (Short, Double) => {
                        self.write_code(OpConvertSWordToDouble);
                    }
                    (UShort, Double) => {
                        self.write_code(OpConvertUWordToDouble);
                    }
                    (Int, Double) => {
                        self.write_code(OpSignExtendDwordToQword);
                        self.write_code(OpConvertSQwordToDouble);
                    }
                    (UInt, Double) => {
                        self.write_code(OpZeroExtendDwordToQword);
                        self.write_code(OpConvertUQwordToDouble);
                    }
                    (Long, Double) => {
                        self.write_code(OpConvertSQwordToDouble);
                    }
                    (ULong, Double) => {
                        self.write_code(OpConvertUQwordToDouble);
                    }
                    (ExtInt, Double) => {
                        self.write_code(OpConvertSOwordToDouble);
                    }
                    (UExtInt, Double) => {
                        self.write_code(OpConvertUOwordToDouble);
                    }
                }
            }
            (ValueType::Float(from), Integer(to)) => {
                match (from, to) {
                    (ValueFloatType::Float, Byte) => {
                        self.write_code(OpConvertFloatToUWord);
                        self.write_code(OpTruncateWordToByte);
                    }
                    (ValueFloatType::Float, SByte) => {
                        self.write_code(OpConvertFloatToSWord);
                        self.write_code(OpTruncateWordToByte);
                    }
                    (ValueFloatType::Float, Short) => {
                        self.write_code(OpConvertFloatToSWord);
                    }
                    (ValueFloatType::Float, UShort) => {
                        self.write_code(OpConvertFloatToUWord);
                    }
                    (ValueFloatType::Float, Int) => {
                        self.write_code(OpConvertFloatToSQword);
                        self.write_code(OpTruncateQwordToDword);
                    }
                    (ValueFloatType::Float, UInt) => {
                        self.write_code(OpConvertFloatToUQword);
                        self.write_code(OpTruncateQwordToDword);
                    }
                    (ValueFloatType::Float, Long) => {
                        self.write_code(OpConvertFloatToSQword);
                    }
                    (ValueFloatType::Float, ULong) => {
                        self.write_code(OpConvertFloatToUQword);
                    }
                    (ValueFloatType::Float, ExtInt) => {
                        self.write_code(OpConvertFloatToSOword);
                    }
                    (ValueFloatType::Float, UExtInt) => {
                        self.write_code(OpConvertFloatToUOword);
                    }
                    (Double, Byte) => {
                        self.write_code(OpConvertDoubleToUWord);
                        self.write_code(OpTruncateWordToByte);
                    }
                    (Double, SByte) => {
                        self.write_code(OpConvertDoubleToSWord);
                        self.write_code(OpTruncateWordToByte);
                    }
                    (Double, Short) => {
                        self.write_code(OpConvertDoubleToSWord);
                    }
                    (Double, UShort) => {
                        self.write_code(OpConvertDoubleToUWord);
                    }
                    (Double, Int) => {
                        self.write_code(OpConvertDoubleToSQword);
                        self.write_code(OpTruncateQwordToDword);
                    }
                    (Double, UInt) => {
                        self.write_code(OpConvertDoubleToUQword);
                        self.write_code(OpTruncateQwordToDword);
                    }
                    (Double, Long) => {
                        self.write_code(OpConvertDoubleToSQword);
                    }
                    (Double, ULong) => {
                        self.write_code(OpConvertDoubleToUQword);
                    }
                    (Double, ExtInt) => {
                        self.write_code(OpConvertDoubleToSOword);
                    }
                    (Double, UExtInt) => {
                        self.write_code(OpConvertDoubleToUOword);
                    }
                }
            }
            (ValueType::Float(from), ValueType::Float(to)) => {
                match (from, to) {
                    (ValueFloatType::Float, ValueFloatType::Float) | (Double, Double) => (),
                    (ValueFloatType::Float, Double) => {
                        self.write_code(OpConvertFloatToDouble);
                    }
                    (Double, ValueFloatType::Float) => {
                        self.write_code(OpConvertDoubleToFloat);
                    }
                }
            }
            (Integer(from), Bool) => {
                match from {
                    Byte | SByte => {
                        self.write_code(OpConvertByteToBool);
                    }
                    Short | UShort => {
                        self.write_code(OpConvertWordToBool);
                    }
                    Int | UInt => {
                        self.write_code(OpConvertDwordToBool);
                    }
                    Long | ULong => {
                        self.write_code(OpConvertQwordToBool);
                    }
                    ExtInt | UExtInt => {
                        self.write_code(OpConvertOwordToBool);
                    }
                }
            }
            (ValueType::Float(from), Bool) => {
                match from {
                    ValueFloatType::Float => {
                        self.write_code(OpConvertDwordToBool);
                    }
                    Double => {
                        self.write_code(OpConvertQwordToBool);
                    }
                }
            }
            (Bool, Integer(_)) |
            (Bool, ValueType::Float(_)) => {
                self.convert_types(&Integer(Byte), to);
            }
            (Bool, Bool) => (),
            _ => panic!("Invalid convert!"),
        }
    }
    
    /// 整型操作指令快捷函数
    #[inline]
    pub fn integer_code(&mut self, 
                        this_type: &ValueType, 
                        byte: Instruction, 
                        word: Instruction, 
                        dword: Instruction, 
                        qword: Instruction, 
                        oword: Instruction) {
        use crate::types::ValueIntegerType::*;
        if let ValueType::Integer(res_type) = this_type {
            match res_type {
                Byte | SByte => self.write_code(byte),
                Short | UShort => self.write_code(word),
                Int | UInt => self.write_code(dword),
                Long | ULong => self.write_code(qword),
                ExtInt | UExtInt => self.write_code(oword),
            }
        } else {
            panic!("Unexpected result type!");
        }
    }
    
    /// 符号相关整型操作指令快捷函数
    #[inline]
    pub fn sign_integer_code(&mut self, 
                             this_type: &ValueType, 
                             signed_byte: Instruction, unsigned_byte: Instruction,
                             signed_word: Instruction, unsigned_word: Instruction,
                             signed_dword: Instruction, unsigned_dword: Instruction,
                             signed_qword: Instruction, unsigned_qword: Instruction,
                             signed_oword: Instruction, unsigned_oword: Instruction) {
        use crate::types::ValueIntegerType::*;
        if let ValueType::Integer(res_type) = this_type {
            match res_type {
                Byte => self.write_code(unsigned_byte),
                SByte => self.write_code(signed_byte),
                Short => self.write_code(signed_word),
                UShort => self.write_code(unsigned_word),
                Int => self.write_code(signed_dword),
                UInt => self.write_code(unsigned_dword),
                Long => self.write_code(signed_qword),
                ULong => self.write_code(unsigned_qword),
                ExtInt => self.write_code(signed_oword),
                UExtInt => self.write_code(unsigned_oword),
            }
        } else {
            panic!("Unexpected result type!");
        }
    }
    
    /// 浮点型操作指令快捷函数
    #[inline]
    pub fn float_code(&mut self, this_type: &ValueType, float: Instruction, double: Instruction) {
        use crate::types::ValueFloatType::*;
        if let ValueType::Float(res_type) = this_type {
            match res_type {
                Float => self.write_code(float),
                Double => self.write_code(double),
            }
        } else {
            panic!("Unexpected result type!");
        }
    }
    
    /// 相反数指令快捷函数
    #[inline]
    pub fn neg_ope_code(&mut self, src_type: &ValueType) {
        match src_type {
            ValueType::Integer(integer) => {
                use crate::types::ValueIntegerType::*;
                match integer {
                    SByte => self.write_code(OpINegByte),
                    Short => self.write_code(OpINegWord),
                    Int => self.write_code(OpINegDword),
                    Long => self.write_code(OpINegQword),
                    ExtInt => self.write_code(OpINegOword),
                    _ => panic!("Unexpected result integer type!"),
                }
            }
            ValueType::Float(float) => {
                use crate::types::ValueFloatType::*;
                match float {
                    Float => self.write_code(OpFNegFloat),
                    Double => self.write_code(OpFNegDouble),
                }
            }
            _ => panic!("Unexpected result type!"),
        }
    }
    
    /// 添加新指令
    #[inline]
    pub fn write_code(&mut self, instr: Instruction) {
        write_byte(&mut self.temp_chunk, [instr.into()]);
    }
    
    /// 添加字节参数
    #[inline]
    pub fn write_arg_byte(&mut self, byte: [u8; 1]) {
        write_byte(&mut self.temp_chunk, byte);
    }
    
    /// 添加单字参数
    #[inline]
    pub fn write_arg_word(&mut self, word: [u8; 2]) {
        write_word(&mut self.temp_chunk, word);
    }
    
    /// 添加双字参数
    #[inline]
    pub fn write_arg_dword(&mut self, dword: [u8; 4]) {
        write_dword(&mut self.temp_chunk, dword);
    }
    
    /// 添加四字参数
    #[inline]
    pub fn write_arg_qword(&mut self, qword: [u8; 8]) {
        write_qword(&mut self.temp_chunk, qword);
    }
    
    /// 添加扩展整数参数
    #[inline]
    pub fn write_arg_oword(&mut self, oword: [u8; 16]) {
        write_oword(&mut self.temp_chunk, oword);
    }
    
    /// 清空临时代码
    #[inline]
    pub fn clear_temp_chunk(&mut self) {
        self.temp_chunk.clear();
    }
    
    /// 附加临时代码
    pub fn append_temp_chunk(&mut self, target: &mut LinkedList<u8>) {
        target.append(&mut self.temp_chunk);
    }
}
