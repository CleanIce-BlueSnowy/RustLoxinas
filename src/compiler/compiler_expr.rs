//! 编译器——表达式模块

use std::rc::Rc;

use crate::compiler::Compiler;
use crate::data::Data;
use crate::expr::{Expr, ExprVisitor};
use crate::instr::Instruction::*;
use crate::position::Position;
use crate::tokens::Token;

impl ExprVisitor<()> for Compiler {
    fn visit_binary_expr(&mut self, this: &Expr, _pos: &Position, left: &Box<Expr>, operator: &Rc<Token>, right: &Box<Expr>) {
        let left_ptr = left.as_ref() as *const Expr;
        let right_ptr = right.as_ref() as *const Expr;
        let this_ptr = this as *const Expr;
        let this_type;
        let temp_type = self.expr_res_type.get(&this_ptr);
        if let Some(temp) = temp_type {
            this_type = temp.clone();
        } else {
            panic!("Unknown expr: {:?}", this_ptr);
        }
        let left_type_temp = self.expr_res_type.get(&left_ptr);
        let right_type_temp = self.expr_res_type.get(&right_ptr);
        let left_type;
        let right_type;
        if let (Some(left_temp), Some(right_temp)) = (left_type_temp, right_type_temp) {
            left_type = left_temp.clone();
            right_type = right_temp.clone();
        } else {
            panic!("Unknown expr: {:?} or {:?}", left_ptr, right_ptr);
        }
        use crate::types::ValueType::*;
        match (&left_type, &right_type) {
            (Integer(_), Integer(_)) => {
                use crate::tokens::TokenType::*;
                left.accept(self);
                self.convert_types(&left_type, &this_type);
                right.accept(self);
                self.convert_types(&right_type, &this_type);
                match &operator.token_type {
                    Operator(ope) => {
                        use crate::tokens::TokenOperator::*;
                        match ope {
                            Plus => self.integer_code(&this_type, OpIAddByte, OpIAddWord, OpIAddDword, OpIAddQword, OpIAddExtInt),
                            Minus => self.integer_code(&this_type, OpISubByte, OpISubWord, OpISubDword, OpISubQword, OpISubExtInt),
                            Star => self.integer_code(&this_type, OpIMulByte, OpIMulWord, OpIMulDword, OpIMulQword, OpIMulExtInt),
                            Slash => self.sign_integer_code(&this_type, OpIDivSByte, OpIDivUByte, OpIDivSWord, OpIDivUWord, OpIDivSDword, OpIDivUDword, OpIDivSQword, OpIDivUQword, OpIDivSExtInt, OpIDivUExtInt),
                            Mod => self.sign_integer_code(&this_type, OpIModSByte, OpIModUByte, OpIModSWord, OpIModUWord, OpIModSDword, OpIModUDword, OpIModSQword, OpIModUQword, OpIModSExtInt, OpIModUExtInt),
                            _ => unimplemented!("Unsupported operation"),
                        }
                    }
                    _ => unimplemented!("Unsupported operation"),
                }
            }
            (Float(_), Float(_)) | (Integer(_), Float(_)) | (Float(_), Integer(_)) => {
                use crate::tokens::TokenType::*;
                left.accept(self);
                self.convert_types(&left_type, &this_type);
                right.accept(self);
                self.convert_types(&right_type, &this_type);
                match &operator.token_type {
                    Operator(ope) => {
                        use crate::tokens::TokenOperator::*;
                        match ope {
                            Plus => self.float_code(&this_type, OpFAddFloat, OpFAddDouble),
                            Minus => self.float_code(&this_type, OpFSubFloat, OpFSubDouble),
                            Star => self.float_code(&this_type, OpFMulFloat, OpFMulDouble),
                            Slash => self.float_code(&this_type, OpFDivFloat, OpFDivDouble),
                            _ => unimplemented!("Unsupported operation"),
                        }
                    }
                    _ => unimplemented!("Unsupported operation"),
                }
            }
            _ => unimplemented!("Unsupported operation"),
        }
    }

    fn visit_grouping_expr(&mut self, _this: &Expr, _pos: &Position, expr: &Box<Expr>) {
        expr.accept(self);
    }

    fn visit_literal_expr(&mut self, _this: &Expr, _pos: &Position, value: &Data) {
        use crate::data::Data::*;
        match value {
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
                        self.write_code(OpLoadConstExtInt);
                        self.write_arg_extend(data.to_le_bytes());
                    }
                    UExtInt(data) => {
                        self.write_code(OpLoadConstExtInt);
                        self.write_arg_extend(data.to_le_bytes());
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
            _ => unimplemented!("Unsupported literal"),
        }
    }

    fn visit_unary_expr(&mut self, _this: &Expr, _pos: &Position, operator: &Rc<Token>, right: &Box<Expr>) {
        let expr_ptr = right.as_ref() as *const Expr;
        let expr_type_temp = self.expr_res_type.get(&expr_ptr);
        let expr_type;
        if let Some(temp) = expr_type_temp {
            expr_type = temp.clone();
        } else {
            panic!("Unexpected expr: {:?}", expr_ptr);
        }
        use crate::types::ValueType::*;
        match expr_type {
            Integer(_) | Float(_) => {
                use crate::tokens::TokenType::*;
                use crate::tokens::TokenOperator::*;
                if let Operator(Minus) = &operator.token_type {
                    right.accept(self);
                    self.neg_ope_code(&expr_type);
                } else {
                    unimplemented!("Unsupported operation");
                }
            }
            _ => unimplemented!("Unsupported operation"),
        }
    }
}
