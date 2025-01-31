//! 语义分析——表达式分析模块

use std::rc::Rc;

use crate::data::{Data, DataFloat, DataInteger};
use crate::expr::{Expr, ExprVisitor};
use crate::object::LoxinasClass;
use crate::position::Position;
use crate::resolver::{CompileError, Resolver, ResolverRes};
use crate::tokens::{Token, TokenKeyword, TokenOperator, TokenType};
use crate::types::{ValueFloatType, ValueType};
use crate::types::ValueType::*;

impl ExprVisitor<Result<ResolverRes, CompileError>> for Resolver {
    fn visit_binary_expr(&mut self, _this: &Expr, pos: &Position, left: &Box<Expr>, operator: &Rc<Token>, right: &Box<Expr>) -> Result<ResolverRes, CompileError> {
        let left_res: ResolverRes = left.accept(self)?;
        let right_res: ResolverRes = right.accept(self)?;
        let left_ptr: *const Expr = left.as_ref() as *const Expr;
        let right_ptr: *const Expr = right.as_ref() as *const Expr;
        self.expr_res_type.insert(left_ptr, left_res.expr_type.clone());
        self.expr_res_type.insert(right_ptr, right_res.expr_type.clone());
        
        // 类型检查
        match (left_res.expr_type, right_res.expr_type) {
            // 两个字符，可以使用 `+` 将其合并为字符串，可以比较
            (Char, Char) => {
                use crate::tokens::TokenOperator::*;
                if let TokenType::Operator(Plus) = &operator.token_type {
                    Ok(ResolverRes::new(Object(LoxinasClass::LoxinasString)))
                } else if let TokenType::Operator(EqualEqual | NotEqual | Less | LessEqual | Greater | GreaterEqual) = &operator.token_type {
                    Ok(ResolverRes::new(Bool))
                } else {
                    Err(CompileError::new(pos, format!("Cannot ues operator '{}' between chars.", Self::operator_to_string(operator))))
                }
            }
            // 两个整数，运算时需要整型提升，操作符不能是布尔运算符
            (Integer(left_type), Integer(right_type)) => {
                use crate::types::ValueIntegerType::*;
                use crate::tokens::TokenKeyword::*;
                if let TokenType::Keyword(And | Or | Not) = &operator.token_type {
                    Err(CompileError::new(pos, format!("Cannot use operator '{}' between integers.", Self::operator_to_string(operator))))
                } else {
                    use crate::tokens::TokenOperator::*;
                    if let TokenType::Operator(EqualEqual | NotEqual | Less | LessEqual | Greater | GreaterEqual) = &operator.token_type {
                        Ok(ResolverRes::new(Bool))
                    } else {
                        match (left_type, right_type) {
                            (SByte, SByte) => Ok(ResolverRes::new(Integer(SByte))),

                            (SByte, Short) |
                            (Short, Short) |
                            (Short, SByte) => Ok(ResolverRes::new(Integer(Short))),

                            (SByte, Int) |
                            (Short, Int) |
                            (Int, Int) |
                            (Int, SByte) |
                            (Int, Short) => Ok(ResolverRes::new(Integer(Int))),

                            (SByte, Long) |
                            (Short, Long) |
                            (Int, Long) |
                            (Long, Long) |
                            (Long, Int) |
                            (Long, Short) |
                            (Long, SByte) => Ok(ResolverRes::new(Integer(Long))),

                            (Byte, ExtInt) |
                            (Short, ExtInt) |
                            (Int, ExtInt) |
                            (Long, ExtInt) |
                            (ExtInt, ExtInt) |
                            (ExtInt, Byte) |
                            (ExtInt, Short) |
                            (ExtInt, Int) |
                            (ExtInt, Long) => Ok(ResolverRes::new(Integer(ExtInt))),

                            (Byte, Byte) => Ok(ResolverRes::new(Integer(Byte))),

                            (Byte, UShort) |
                            (UShort, UShort) |
                            (UShort, Byte) => Ok(ResolverRes::new(Integer(UShort))),

                            (Byte, UInt) |
                            (UShort, UInt) |
                            (UInt, UInt) |
                            (UInt, Byte) |
                            (UInt, UShort) => Ok(ResolverRes::new(Integer(UInt))),

                            (Byte, ULong) |
                            (UShort, ULong) |
                            (UInt, ULong) |
                            (ULong, ULong) |
                            (ULong, UInt) |
                            (ULong, UShort) |
                            (ULong, Byte) => Ok(ResolverRes::new(Integer(ULong))),

                            (Byte, UExtInt) |
                            (UShort, UExtInt) |
                            (UInt, UExtInt) |
                            (ULong, UExtInt) |
                            (UExtInt, UExtInt) |
                            (UExtInt, Byte) |
                            (UExtInt, UShort) |
                            (UExtInt, UInt) |
                            (UExtInt, ULong) => Ok(ResolverRes::new(Integer(UExtInt))),

                            _ => Err(CompileError::new(pos, "Cannot operate on two integers with different signs.".to_string())),
                        }
                    }
                }
            }
            // 两个数字，其中一个是浮点数，结果提升为浮点数，操作符不能是布尔运算符和取模运算符
            (Integer(_), Float(float)) |
            (Float(float), Integer(_)) => {
                use crate::tokens::TokenKeyword::*;
                use crate::tokens::TokenOperator::*;
                if let TokenType::Keyword(TokenKeyword::And | Or) = &operator.token_type {
                    Err(CompileError::new(pos, "Cannot use operator '{}' between numbers.".to_string()))
                } else if let TokenType::Operator(EqualEqual | NotEqual | Less | LessEqual | Greater | GreaterEqual) = &operator.token_type {
                    Ok(ResolverRes::new(Bool))
                } else if let TokenType::Operator(Mod) = &operator.token_type {
                    Err(CompileError::new(pos, "Cannot use operator '%' on a floating-point number.".to_string()))
                } else {
                    Ok(ResolverRes::new(Float(float)))
                }
            }
            // 两个浮点数，需要提升，操作符不能是布尔运算符
            (Float(left_type), Float(right_type)) => {
                use crate::types::ValueFloatType::*;
                use crate::tokens::TokenKeyword::*;
                use crate::tokens::TokenOperator::*;
                if let TokenType::Keyword(TokenKeyword::And | Or) = &operator.token_type {
                    Err(CompileError::new(pos, "Cannot use operator '{}' between floating-point numbers.".to_string()))
                } else if let TokenType::Operator(EqualEqual | NotEqual | Less | LessEqual | Greater | GreaterEqual) = &operator.token_type {
                    Ok(ResolverRes::new(Bool))
                } else if let TokenType::Operator(Mod) = &operator.token_type {
                    Err(CompileError::new(pos, "Cannot use operator '%' between floating-point numbers.".to_string()))
                } else {
                    Ok(ResolverRes::new(ValueType::Float(
                        match (left_type, right_type) {
                            (Float, Float) => Float,
                            _ => Double,
                        }
                    )))
                }
            }
            // 两个布尔型，只支持布尔运算符、判等和不等号
            (Bool, Bool) => {
                use crate::tokens::TokenType::*;
                match &operator.token_type {
                    Operator(ope) => {
                        use crate::tokens::TokenOperator::*;
                        if let EqualEqual | NotEqual = ope {
                            Ok(ResolverRes::new(Bool))
                        } else {
                            Err(CompileError::new(pos, format!("Cannot use operator '{}' between bools.", Self::operator_to_string(operator))))
                        }
                    }
                    Keyword(ope) => {
                        use crate::tokens::TokenKeyword::*;
                        if let And | Or = ope {
                            Ok(ResolverRes::new(Bool))
                        } else {
                            Err(CompileError::new(pos, format!("Cannot use operator '{}' between bools.", Self::operator_to_string(operator))))
                        }
                    }
                    _ => panic!("Invalid operator")
                }
            }
            // 两个对象，如果是字符串则可以拼接或比较，否则无效
            (Object(left_type), Object(right_type)) => {
                if let (LoxinasClass::LoxinasString, LoxinasClass::LoxinasString) = (left_type, right_type) {
                    use crate::tokens::TokenType::*;
                    match &operator.token_type {
                        Operator(ope) => {
                            use crate::tokens::TokenOperator::*;
                            if let Plus = ope {
                                Ok(ResolverRes::new(Object(LoxinasClass::LoxinasString)))
                            } else if let EqualEqual | NotEqual | Less | LessEqual | Greater | GreaterEqual = ope {
                                Ok(ResolverRes::new(Bool))
                            } else {
                                Err(CompileError::new(pos, format!("Cannot use operator '{}' between strings.", Self::operator_to_string(operator))))
                            }
                        }
                        _ => Err(CompileError::new(pos, format!("Cannot use operator '{}' between strings.", Self::operator_to_string(operator))))
                    }
                } else {
                    Err(CompileError::new(pos, "Cannot operate on two objects.".to_string()))
                }
            }
            // 其余组合均无效
            (left_type, right_type) => {
                Err(CompileError::new(pos, format!("Cannot use operator '{}' between '{}' and '{}'", Self::operator_to_string(operator), left_type, right_type)))
            }
        }
    }

    fn visit_grouping_expr(&mut self, _this: &Expr, _pos: &Position, expr: &Box<Expr>) -> Result<ResolverRes, CompileError> {
        let expr_res: ResolverRes = expr.accept(self)?;
        let expr_ptr = expr.as_ref() as *const Expr;
        self.expr_res_type.insert(expr_ptr, expr_res.expr_type.clone());
        return Ok(expr_res);
    }

    fn visit_literal_expr(&mut self, _this: &Expr, _pos: &Position, value: &Data) -> Result<ResolverRes, CompileError> {
        // 直接返回值的类型即可
        Ok(ResolverRes::new(
            match value {
                Data::Char(_) => Char,
                Data::Bool(_) => Bool,
                Data::Float(float) => {
                    match float {
                        DataFloat::Float(_) => Float(ValueFloatType::Float),
                        DataFloat::Double(_) => Float(ValueFloatType::Double),
                    }
                }
                Data::Integer(integer) => {
                    Integer({
                        use crate::types::ValueIntegerType::*;
                        match integer {
                            DataInteger::Byte(_) => Byte,
                            DataInteger::SByte(_) => SByte,
                            DataInteger::Short(_) => Short,
                            DataInteger::UShort(_) => UShort,
                            DataInteger::Int(_) => Int,
                            DataInteger::UInt(_) => UInt,
                            DataInteger::Long(_) => Long,
                            DataInteger::ULong(_) => ULong,
                            DataInteger::ExtInt(_) => ExtInt,
                            DataInteger::UExtInt(_) => UExtInt,
                        }
                    })
                }
                Data::String(_) => {
                    Object(LoxinasClass::LoxinasString)
                }
            }
        ))
    }

    fn visit_unary_expr(&mut self, _this: &Expr, pos: &Position, operator: &Rc<Token>, right: &Box<Expr>) -> Result<ResolverRes, CompileError> {
        let expr_res: ResolverRes = right.accept(self)?;
        let expr_ptr: *const Expr = right.as_ref() as *const Expr;
        self.expr_res_type.insert(expr_ptr, expr_res.expr_type.clone());
        match expr_res.expr_type {
            // 整数，结果为原类型，不允许布尔运算符，无符号整数不允许相反数（补码）
            Integer(integer) => {
                if let TokenType::Keyword(TokenKeyword::Not) = &operator.token_type {
                    Err(CompileError::new(pos, format!("Cannot use operator '{}' on an integer.", Self::operator_to_string(operator))))
                } else {
                    use crate::types::ValueIntegerType::*;
                    if let Byte | UShort | UInt | ULong | UExtInt = integer {
                        if let TokenType::Operator(TokenOperator::Minus) = operator.token_type {
                            return Err(CompileError::new(pos, "Cannot use operator '-' on an unsigned integer.".to_string()));
                        }
                    }
                    Ok(ResolverRes::new(Integer(integer)))
                }
            }
            // 浮点数，结果为原类型，不允许布尔运算符
            Float(float) => {
                if let TokenType::Keyword(TokenKeyword::Not) = &operator.token_type {
                    Err(CompileError::new(pos, format!("Cannot use operator '{}' on a floating-point number.", Self::operator_to_string(operator))))
                } else {
                    Ok(ResolverRes::new(Float(float)))
                }
            }
            // 布尔型，结果为布尔型，只允许逻辑非运算符
            Bool => {
                if let TokenType::Keyword(TokenKeyword::Not) = &operator.token_type {
                    Ok(ResolverRes::new(Bool))
                } else {
                    Err(CompileError::new(pos, format!("Cannot use operator '{}' on a bool.", Self::operator_to_string(operator))))
                }
            }
            // 其他类型均无效
            expr_type => Err(CompileError::new(pos, format!("Cannot use '{}' on a '{}'", Self::operator_to_string(operator), expr_type)))
        }
    }
}
