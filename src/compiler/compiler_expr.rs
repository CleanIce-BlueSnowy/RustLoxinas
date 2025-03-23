//! 编译器——表达式模块

use std::collections::LinkedList;

use crate::compiler::Compiler;
use crate::data::DataSize;
use crate::errors::error_types::{CompileError, CompileResult};
use crate::expr::{ExprBinary, ExprLiteral, ExprUnary};
use crate::instr::Instruction::*;
use crate::resolver::ExprResolveRes;
use crate::tokens::{TokenKeyword, TokenType};

impl Compiler {
    /// 编译二元运算表达式
    pub fn compile_binary_expr(&mut self, 
                               expr: &ExprBinary,
                               resolve_res: &ExprResolveRes,
                               left_code: &mut LinkedList<u8>,
                               left_res: &ExprResolveRes,
                               right_code: &mut LinkedList<u8>,
                               right_res: &ExprResolveRes) -> CompileResult<LinkedList<u8>> {
        let mut target = LinkedList::new();
        
        // 位移走单独路线
        if let TokenType::Keyword(TokenKeyword::Shl | TokenKeyword::Shr) = &expr.operator.token_type {
            target.append(left_code);
            target.append(right_code);
            let this_type = &resolve_res.ope_type;
            if let TokenType::Keyword(TokenKeyword::Shl) = &expr.operator.token_type {
                self.integer_code(this_type, OpShiftLeftByte, OpShiftLeftWord, OpShiftLeftDword, OpShiftLeftQword, OpShiftLeftOword);
            } else {  // TokenType::Keyword(TokenKeyword::Rsh)
                self.sign_integer_code(this_type, OpSignShiftRightByte, OpZeroShiftRightByte, OpSignShiftRightWord, OpZeroShiftRightWord, OpSignShiftRightDword, OpZeroShiftRightDword, OpSignShiftRightQword, OpZeroShiftRightQword, OpSignShiftRightOword, OpZeroShiftRightOword);
            }
            self.append_temp_chunk(&mut target);
            return Ok(target);
        }
        
        // 获取类型并粘合代码
        let this_type = &resolve_res.ope_type;
        let left_type = &left_res.res_type;
        target.append(left_code);
        self.convert_types(left_type, this_type);
        self.append_temp_chunk(&mut target);
        let right_type = &right_res.res_type;
        target.append(right_code);
        self.convert_types(right_type, this_type);
        self.append_temp_chunk(&mut target);
        
        use crate::types::ValueType::*;
        match (&left_type, &right_type) {
            (Integer(_), Integer(_)) => {
                use crate::tokens::TokenType::*;
                
                match &expr.operator.token_type {
                    Operator(ope) => {
                        use crate::tokens::TokenOperator::*;
                        match ope {
                            Plus => self.integer_code(this_type, OpIAddByte, OpIAddWord, OpIAddDword, OpIAddQword, OpIAddOword),
                            Minus => self.integer_code(this_type, OpISubByte, OpISubWord, OpISubDword, OpISubQword, OpISubOword),
                            Star => self.integer_code(this_type, OpIMulByte, OpIMulWord, OpIMulDword, OpIMulQword, OpIMulOword),
                            Slash => self.sign_integer_code(this_type, OpIDivSByte, OpIDivUByte, OpIDivSWord, OpIDivUWord, OpIDivSDword, OpIDivUDword, OpIDivSQword, OpIDivUQword, OpIDivSOword, OpIDivUOword),
                            Mod => self.sign_integer_code(this_type, OpIModSByte, OpIModUByte, OpIModSWord, OpIModUWord, OpIModSDword, OpIModUDword, OpIModSQword, OpIModUQword, OpIModSOword, OpIModUOword),
                            And => self.integer_code(this_type, OpBitAndByte, OpBitAndWord, OpBitAndDword, OpBitAndQword, OpBitAndOword),
                            Pipe => self.integer_code(this_type, OpBitOrByte, OpBitOrWord, OpBitOrDword, OpBitOrQword, OpBitOrOword),
                            Caret => self.integer_code(this_type, OpBitXorByte, OpBitXorWord, OpBitXorDword, OpBitXorQword, OpBitXorOword),
                            EqualEqual => self.integer_code(this_type, OpICmpEqualByte, OpICmpEqualWord, OpICmpEqualDword, OpICmpEqualQword, OpICmpEqualOword),
                            NotEqual => self.integer_code(this_type, OpICmpNotEqualByte, OpICmpNotEqualWord, OpICmpNotEqualDword, OpICmpNotEqualQword, OpICmpNotEqualOword),
                            Less => self.sign_integer_code(this_type, OpICmpLessSByte, OpICmpLessUByte, OpICmpLessSWord, OpICmpLessUWord, OpICmpLessSDword, OpICmpLessUDword, OpICmpLessSQword, OpICmpLessUQword, OpICmpLessSOword, OpICmpLessUOword),
                            LessEqual => self.sign_integer_code(this_type, OpICmpLessEqualSByte, OpICmpLessEqualUByte, OpICmpLessEqualSWord, OpICmpLessEqualUWord, OpICmpLessEqualSDword, OpICmpLessEqualUDword, OpICmpLessEqualSQword, OpICmpLessEqualUQword, OpICmpLessEqualSOword, OpICmpLessEqualUOword),
                            Greater => self.sign_integer_code(this_type, OpICmpGreaterSByte, OpICmpGreaterUByte, OpICmpGreaterSWord, OpICmpGreaterUWord, OpICmpGreaterSDword, OpICmpGreaterUDword, OpICmpGreaterSQword, OpICmpGreaterUQword, OpICmpGreaterSOword, OpICmpGreaterUOword),
                            GreaterEqual => self.sign_integer_code(this_type, OpICmpGreaterEqualSByte, OpICmpGreaterEqualUByte, OpICmpGreaterEqualSWord, OpICmpGreaterEqualUWord, OpICmpGreaterEqualSDword, OpICmpGreaterEqualUDword, OpICmpGreaterEqualSQword, OpICmpGreaterEqualUQword, OpICmpGreaterEqualSOword, OpICmpGreaterEqualUOword),
                            _ => unimplemented!("Unsupported operation"),
                        }
                    }
                    _ => unimplemented!("Unsupported operation"),
                }
            }
            (Float(_), Float(_)) | (Integer(_), Float(_)) | (Float(_), Integer(_)) => {
                use crate::tokens::TokenType::*;
                
                match &expr.operator.token_type {
                    Operator(ope) => {
                        use crate::tokens::TokenOperator::*;
                        match ope {
                            Plus => self.float_code(this_type, OpFAddFloat, OpFAddDouble),
                            Minus => self.float_code(this_type, OpFSubFloat, OpFSubDouble),
                            Star => self.float_code(this_type, OpFMulFloat, OpFMulDouble),
                            Slash => self.float_code(this_type, OpFDivFloat, OpFDivDouble),
                            EqualEqual => self.float_code(this_type, OpFCmpEqualFloat, OpFCmpEqualDouble),
                            NotEqual => self.float_code(this_type, OpFCmpNotEqualFloat, OpFCmpNotEqualDouble),
                            Less => self.float_code(this_type, OpFCmpLessFloat, OpFCmpLessDouble),
                            LessEqual => self.float_code(this_type, OpFCmpLessEqualFloat, OpFCmpLessEqualDouble),
                            Greater => self.float_code(this_type, OpFCmpGreaterFloat, OpFCmpGreaterDouble),
                            GreaterEqual => self.float_code(this_type, OpFCmpGreaterEqualFloat, OpFCmpGreaterEqualDouble),
                            _ => unimplemented!("Unsupported operation"),
                        }
                    }
                    _ => unimplemented!("Unsupported operation"),
                }
            }
            _ => unimplemented!("Unsupported operation"),
        }
        
        self.append_temp_chunk(&mut target);
        
        return Ok(target);
    }

    /// 编译分组表达式
    pub fn compile_grouping_expr(&mut self,
                                 inside_code: &mut LinkedList<u8>) -> Result<LinkedList<u8>, CompileError> {
        let mut target = LinkedList::new();
        target.append(inside_code);
        return Ok(target);
    }

    /// 编译字面量表达式
    pub fn compile_literal_expr(&mut self,
                                expr: &ExprLiteral) -> Result<LinkedList<u8>, CompileError> {
        use crate::data::Data::*;
        let mut target = LinkedList::new();
        
        match &expr.value {
            Integer(integer) => {
                use crate::data::DataInteger::*;
                match integer {
                    Byte(data) => {
                        self.write_code(OpLoadConstByte);
                        self.write_arg_byte(data.to_le_bytes());
                    }
                    SByte(data) => {
                        self.write_code(OpLoadConstByte);
                        self.write_arg_byte(data.to_le_bytes());
                    }
                    Short(data) => {
                        self.write_code(OpLoadConstWord);
                        self.write_arg_word(data.to_le_bytes());
                    }
                    UShort(data) => {
                        self.write_code(OpLoadConstWord);
                        self.write_arg_word(data.to_le_bytes());
                    }
                    Int(data) => {
                        self.write_code(OpLoadConstDword);
                        self.write_arg_dword(data.to_le_bytes());
                    }
                    UInt(data) => {
                        self.write_code(OpLoadConstDword);
                        self.write_arg_dword(data.to_le_bytes());
                    }
                    Long(data) => {
                        self.write_code(OpLoadConstQword);
                        self.write_arg_qword(data.to_le_bytes());
                    }
                    ULong(data) => {
                        self.write_code(OpLoadConstQword);
                        self.write_arg_qword(data.to_le_bytes());
                    }
                    ExtInt(data) => {
                        self.write_code(OpLoadConstOword);
                        self.write_arg_oword(data.to_le_bytes());
                    }
                    UExtInt(data) => {
                        self.write_code(OpLoadConstOword);
                        self.write_arg_oword(data.to_le_bytes());
                    }
                }
            }
            Float(float) => {
                use crate::data::DataFloat::*;
                match float {
                    Float(data) => {
                        self.write_code(OpLoadConstDword);
                        self.write_arg_dword(data.to_le_bytes());
                    }
                    Double(data) => {
                        self.write_code(OpLoadConstQword);
                        self.write_arg_qword(data.to_le_bytes());
                    }
                }
            }
            Char(char) => {
                self.write_code(OpLoadConstDword);
                self.write_arg_dword((*char as u32).to_le_bytes());
            }
            _ => unimplemented!("Unsupported literal"),
        }
        
        self.append_temp_chunk(&mut target);
        
        return Ok(target);
    }

    /// 编译单元运算表达式
    pub fn compile_unary_expr(&mut self,
                              expr: &ExprUnary,
                              right_code: &mut LinkedList<u8>,
                              right_res: &ExprResolveRes) -> Result<LinkedList<u8>, CompileError> {
        let mut target = LinkedList::new();
        
        let expr_type = &right_res.res_type;
        target.append(right_code);
        
        use crate::types::ValueType::*;
        match expr_type {
            Integer(_) | Float(_) => {
                use crate::tokens::TokenType::*;
                use crate::tokens::TokenOperator::*;
                if let Operator(Minus) = &expr.operator.token_type {
                    self.neg_ope_code(&expr_type);
                } else if let Operator(Tilde) = &expr.operator.token_type {
                    self.integer_code(&expr_type, OpBitNotByte, OpBitNotWord, OpBitNotDword, OpBitNotQword, OpBitNotOword);
                } else {
                    unimplemented!("Unsupported operation");
                }
            }
            _ => unimplemented!("Unsupported operation"),
        }
        
        self.append_temp_chunk(&mut target);
        
        return Ok(target);
    }

    /// 编译转换表达式
    pub fn compile_as_expr(&mut self,
                           resolve_res: &ExprResolveRes,
                           inside_code: &mut LinkedList<u8>) -> Result<LinkedList<u8>, CompileError> {
        // 直接计算并转换
        let mut target = LinkedList::new();
        let ope_type = &resolve_res.ope_type;
        let res_type = &resolve_res.res_type;
        target.append(inside_code);
        self.convert_types(ope_type, res_type);
        self.append_temp_chunk(&mut target);
        
        return Ok(target);
    }
    
    pub fn compile_variable_expr(&mut self,
                                 resolve_res: &ExprResolveRes,
                                 slot: usize, 
                                 in_assign: bool,
                                 in_ref_let: bool,
                                 is_ref: bool) -> CompileResult<LinkedList<u8>> {
        let mut target = LinkedList::new();
        
        // 引用中，直接返回偏移量
        if in_ref_let {
            self.write_code(OpPushWord);
            self.write_arg_word((slot as u16).to_le_bytes());
        } else {
            self.write_code(
                if is_ref {
                    if in_assign {
                        match resolve_res.res_type.get_size() {
                            DataSize::Byte => OpSetReferenceByte,
                            DataSize::Word => OpSetReferenceWord,
                            DataSize::Dword => OpSetReferenceDword,
                            DataSize::Qword => OpSetReferenceQword,
                            DataSize::Oword => OpSetReferenceOword,
                        }
                    } else {
                        match resolve_res.res_type.get_size() {
                            DataSize::Byte => OpGetReferenceByte,
                            DataSize::Word => OpGetReferenceWord,
                            DataSize::Dword => OpGetReferenceDword,
                            DataSize::Qword => OpGetReferenceQword,
                            DataSize::Oword => OpGetReferenceOword,
                        }
                    }
                } else {
                    if in_assign {
                        match resolve_res.res_type.get_size() {
                            DataSize::Byte => OpSetLocalByte,
                            DataSize::Word => OpSetLocalWord,
                            DataSize::Dword => OpSetLocalDword,
                            DataSize::Qword => OpSetLocalQword,
                            DataSize::Oword => OpSetLocalOword,
                        }
                    } else {
                        match resolve_res.res_type.get_size() {
                            DataSize::Byte => OpGetLocalByte,
                            DataSize::Word => OpGetLocalWord,
                            DataSize::Dword => OpGetLocalDword,
                            DataSize::Qword => OpGetLocalQword,
                            DataSize::Oword => OpGetLocalOword,
                        }
                    }
                }
            );
            self.write_arg_word((slot as u16).to_le_bytes());
        }
        
        self.append_temp_chunk(&mut target);
        
        return Ok(target);
    }
}
