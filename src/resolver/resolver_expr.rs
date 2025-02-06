//! 语义分析——表达式分析模块

use std::rc::Rc;

use crate::data::{Data, DataFloat, DataInteger};
use crate::expr::{Expr, ExprVisitor};
use crate::object::LoxinasClass;
use crate::position::Position;
use crate::resolver::{CompileError, ExprResolveRes, Resolver};
use crate::tokens::{Token, TokenKeyword, TokenOperator, TokenType};
use crate::types::{TypeTag, ValueFloatType, ValueType};
use crate::types::ValueType::*;

impl ExprVisitor<Result<ExprResolveRes, CompileError>> for Resolver {
    fn visit_binary_expr(&mut self, this: &Expr, pos: &Position, left: &Box<Expr>, operator: &Rc<Token>, right: &Box<Expr>) -> Result<ExprResolveRes, CompileError> {
        let left_res: ExprResolveRes = left.accept(self)?;
        let right_res: ExprResolveRes = right.accept(self)?;
        let res_type;
        let ope_type;
        
        // 类型检查
        match (left_res.res_type, right_res.res_type) {
            // 两个字符，可以使用 `+` 将其合并为字符串，可以比较
            (Char, Char) => {
                ope_type = Char;
                use crate::tokens::TokenOperator::*;
                if let TokenType::Operator(Plus) = &operator.token_type {
                    res_type = Object(LoxinasClass::String);
                } else if let TokenType::Operator(EqualEqual | NotEqual | Less | LessEqual | Greater | GreaterEqual) = &operator.token_type {
                    res_type = Bool;
                } else {
                    return Err(CompileError::new(pos, format!("Cannot ues operator '{}' between chars.", Self::operator_to_string(operator))));
                }
            }
            // 两个整数，运算时需要整型提升，操作符不能是布尔运算符
            (Integer(left_type), Integer(right_type)) => {
                use crate::types::ValueIntegerType::*;
                use crate::tokens::TokenKeyword::*;
                if let TokenType::Keyword(And | Or | Not) = &operator.token_type {
                    return Err(CompileError::new(pos, format!("Cannot use operator '{}' between integers.", Self::operator_to_string(operator))));
                } else {
                    use crate::tokens::TokenOperator::*;
                    ope_type = match (left_type, right_type) {
                        (SByte, SByte) => Integer(SByte),

                        (SByte, Short) |
                        (Short, Short) |
                        (Short, SByte) => Integer(Short),

                        (SByte, Int) |
                        (Short, Int) |
                        (Int, Int) |
                        (Int, SByte) |
                        (Int, Short) => Integer(Int),

                        (SByte, Long) |
                        (Short, Long) |
                        (Int, Long) |
                        (Long, Long) |
                        (Long, Int) |
                        (Long, Short) |
                        (Long, SByte) => Integer(Long),

                        (Byte, ExtInt) |
                        (Short, ExtInt) |
                        (Int, ExtInt) |
                        (Long, ExtInt) |
                        (ExtInt, ExtInt) |
                        (ExtInt, Byte) |
                        (ExtInt, Short) |
                        (ExtInt, Int) |
                        (ExtInt, Long) => Integer(ExtInt),

                        (Byte, Byte) => Integer(Byte),

                        (Byte, UShort) |
                        (UShort, UShort) |
                        (UShort, Byte) => Integer(UShort),

                        (Byte, UInt) |
                        (UShort, UInt) |
                        (UInt, UInt) |
                        (UInt, Byte) |
                        (UInt, UShort) => Integer(UInt),

                        (Byte, ULong) |
                        (UShort, ULong) |
                        (UInt, ULong) |
                        (ULong, ULong) |
                        (ULong, UInt) |
                        (ULong, UShort) |
                        (ULong, Byte) => Integer(ULong),

                        (Byte, UExtInt) |
                        (UShort, UExtInt) |
                        (UInt, UExtInt) |
                        (ULong, UExtInt) |
                        (UExtInt, UExtInt) |
                        (UExtInt, Byte) |
                        (UExtInt, UShort) |
                        (UExtInt, UInt) |
                        (UExtInt, ULong) => Integer(UExtInt),

                        _ => return Err(CompileError::new(pos, "Cannot operate on two integers with different signs.".to_string())),
                    };
                    res_type = if let TokenType::Operator(EqualEqual | NotEqual | Less | LessEqual | Greater | GreaterEqual) = &operator.token_type {
                        Bool
                    } else {
                        ope_type.clone()
                    };
                }
            }
            // 两个数字，其中一个是浮点数，结果提升为浮点数，操作符不能是布尔运算符、取模运算符以及位运算符
            (Integer(_), Float(float)) |
            (Float(float), Integer(_)) => {
                ope_type = Float(float.clone());
                use crate::tokens::TokenKeyword::*;
                use crate::tokens::TokenOperator::*;
                if let TokenType::Keyword(TokenKeyword::And | Or) = &operator.token_type {
                    return Err(CompileError::new(pos, "Cannot use operator '{}' between numbers.".to_string()));
                } else if let TokenType::Operator(TokenOperator::And | Pipe | Caret) = &operator.token_type {
                    return Err(CompileError::new(pos, "Cannot use operator '{}' between floating-point numbers.".to_string()));
                } else if let TokenType::Operator(EqualEqual | NotEqual | Less | LessEqual | Greater | GreaterEqual) = &operator.token_type {
                    res_type = Bool;
                } else if let TokenType::Operator(Mod) = &operator.token_type {
                    return Err(CompileError::new(pos, "Cannot use operator '%' on a floating-point number.".to_string()));
                } else {
                    res_type = Float(float);
                }
            }
            // 两个浮点数，需要提升，操作符不能是布尔运算符、取模运算符和位运算符
            (Float(left_type), Float(right_type)) => {
                use crate::types::ValueFloatType::*;
                use crate::tokens::TokenKeyword::*;
                use crate::tokens::TokenOperator::*;
                ope_type = ValueType::Float(match (left_type, right_type) {
                    (Float, Float) => Float,
                    _ => Double,
                });
                if let TokenType::Keyword(TokenKeyword::And | Or) = &operator.token_type {
                    return Err(CompileError::new(pos, "Cannot use operator '{}' between floating-point numbers.".to_string()));
                } else if let TokenType::Operator(TokenOperator::And | Pipe | Caret) = &operator.token_type {
                    return Err(CompileError::new(pos, "Cannot use operator '{}' between floating-point numbers.".to_string()));
                } else if let TokenType::Operator(EqualEqual | NotEqual | Less | LessEqual | Greater | GreaterEqual) = &operator.token_type {
                    res_type = Bool;
                } else if let TokenType::Operator(Mod) = &operator.token_type {
                    return Err(CompileError::new(pos, "Cannot use operator '%' between floating-point numbers.".to_string()));
                } else {
                    res_type = ope_type.clone();
                }
            }
            // 两个布尔型，只支持布尔运算符、判等和不等号
            (Bool, Bool) => {
                use crate::tokens::TokenType::*;
                ope_type = Bool;
                match &operator.token_type {
                    Operator(ope) => {
                        use crate::tokens::TokenOperator::*;
                        if let EqualEqual | NotEqual = ope {
                            res_type = Bool;
                        } else {
                            return Err(CompileError::new(pos, format!("Cannot use operator '{}' between bools.", Self::operator_to_string(operator))));
                        }
                    }
                    Keyword(ope) => {
                        use crate::tokens::TokenKeyword::*;
                        if let And | Or = ope {
                            res_type = Bool;
                        } else {
                            return Err(CompileError::new(pos, format!("Cannot use operator '{}' between bools.", Self::operator_to_string(operator))));
                        }
                    }
                    _ => panic!("Invalid operator")
                }
            }
            // 两个对象，如果是字符串则可以拼接或比较，否则无效
            (Object(left_type), Object(right_type)) => {
                ope_type = Object(LoxinasClass::Object);
                if let (LoxinasClass::String, LoxinasClass::String) = (left_type, right_type) {
                    use crate::tokens::TokenType::*;
                    match &operator.token_type {
                        Operator(ope) => {
                            use crate::tokens::TokenOperator::*;
                            if let Plus = ope {
                                res_type = Object(LoxinasClass::String);
                            } else if let EqualEqual | NotEqual | Less | LessEqual | Greater | GreaterEqual = ope {
                                res_type = Bool;
                            } else {
                                return Err(CompileError::new(pos, format!("Cannot use operator '{}' between strings.", Self::operator_to_string(operator))));
                            }
                        }
                        _ => return Err(CompileError::new(pos, format!("Cannot use operator '{}' between strings.", Self::operator_to_string(operator))))
                    }
                } else {
                    return Err(CompileError::new(pos, "Cannot operate on two objects.".to_string()))
                }
            }
            // 其余组合均无效
            (left_type, right_type) => {
                return Err(CompileError::new(pos, format!("Cannot use operator '{}' between '{}' and '{}'", Self::operator_to_string(operator), left_type, right_type)))
            }
        }

        // 保存当前表达式信息
        let this_ptr = this as *const Expr;
        self.expr_ope_type.insert(this_ptr, ope_type.clone());
        self.expr_res_type.insert(this_ptr, res_type.clone());
        return Ok(ExprResolveRes::new(res_type, ope_type));
    }

    fn visit_grouping_expr(&mut self, this: &Expr, _pos: &Position, expr: &Box<Expr>) -> Result<ExprResolveRes, CompileError> {
        let expr_res: ExprResolveRes = expr.accept(self)?;

        // 保存当前表达式信息
        let this_ptr = this as *const Expr;
        self.expr_ope_type.insert(this_ptr, expr_res.ope_type.clone());
        self.expr_res_type.insert(this_ptr, expr_res.res_type.clone());
        return Ok(expr_res);
    }

    fn visit_literal_expr(&mut self, this: &Expr, _pos: &Position, value: &Data) -> Result<ExprResolveRes, CompileError> {
        // 直接返回值的类型即可
        let ope_type = match value {
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
                Object(LoxinasClass::String)
            }
        };
        let res_type = ope_type.clone();

        // 保存当前表达式信息
        let this_ptr = this as *const Expr;
        self.expr_ope_type.insert(this_ptr, ope_type.clone());
        self.expr_res_type.insert(this_ptr, res_type.clone());
        return Ok(ExprResolveRes::new(res_type, ope_type));
    }

    fn visit_unary_expr(&mut self, this: &Expr, pos: &Position, operator: &Rc<Token>, right: &Box<Expr>) -> Result<ExprResolveRes, CompileError> {
        let expr_res: ExprResolveRes = right.accept(self)?;
        let res_type;
        let ope_type;
        match expr_res.res_type {
            // 整数，结果为原类型，不允许布尔运算符，无符号整数不允许相反数（补码）
            Integer(integer) => {
                if let TokenType::Keyword(TokenKeyword::Not) = &operator.token_type {
                    return Err(CompileError::new(pos, format!("Cannot use operator '{}' on an integer.", Self::operator_to_string(operator))));
                } else {
                    use crate::types::ValueIntegerType::*;
                    if let Byte | UShort | UInt | ULong | UExtInt = integer {
                        if let TokenType::Operator(TokenOperator::Minus) = operator.token_type {
                            return Err(CompileError::new(pos, "Cannot use operator '-' on an unsigned integer.".to_string()));
                        }
                    }
                    ope_type = Integer(integer);
                }
            }
            // 浮点数，结果为原类型，不允许布尔运算符和位运算符
            Float(float) => {
                if let TokenType::Keyword(TokenKeyword::Not) | TokenType::Operator(TokenOperator::Tilde) = &operator.token_type {
                    return Err(CompileError::new(pos, format!("Cannot use operator '{}' on a floating-point number.", Self::operator_to_string(operator))));
                } else {
                    ope_type = Float(float)
                }
            }
            // 布尔型，结果为布尔型，只允许逻辑非运算符
            Bool => {
                if let TokenType::Keyword(TokenKeyword::Not) = &operator.token_type {
                    ope_type = Bool;
                } else {
                    return Err(CompileError::new(pos, format!("Cannot use operator '{}' on a bool.", Self::operator_to_string(operator))));
                }
            }
            // 其他类型均无效
            expr_type => return Err(CompileError::new(pos, format!("Cannot use '{}' on a '{}'", Self::operator_to_string(operator), expr_type)))
        }
        res_type = ope_type.clone();

        // 保存当前表达式信息
        let this_ptr = this as *const Expr;
        self.expr_ope_type.insert(this_ptr, ope_type.clone());
        self.expr_res_type.insert(this_ptr, res_type.clone());
        return Ok(ExprResolveRes::new(res_type, ope_type));
    }

    fn visit_as_expr(&mut self, this: &Expr, pos: &Position, expr: &Box<Expr>, target: &TypeTag) -> Result<ExprResolveRes, CompileError> {
        let expr_res: ExprResolveRes = expr.accept(self)?;
        let ope_type = expr_res.res_type;
        
        // 不允许在对象上使用 `as`
        if let Object(_) = ope_type {
            return Err(CompileError::new(pos, format!("Cannot use 'as' on a value of type '{}'.", ope_type)));
        }
        
        // 链式查找类型
        let mut res_type: Option<ValueType> = None;
        let mut search_map = self.global_types.clone();
        let mut in_global = true;
        for name in &target.chain {
            if let Some(temp_ty) = &res_type {
                if let Object(object) = temp_ty {
                    search_map = object.get_contain_types().clone();
                    in_global = false;
                } else {
                    return Err(CompileError::new(&target.pos, format!("Unknown type '{}' in '{}'.", name, temp_ty)));
                }
            }
            let ty = if let Some(temp) = search_map.get(name) {
                temp
            } else {
                return Err(CompileError::new(&target.pos,
                                             if in_global {
                                                 format!("Unknown type '{}' in global.", name)
                                             } else {
                                                 format!("Unknown type '{}' in '{}'.", name, res_type.as_ref().unwrap())
                                             }));
            };
            res_type = Some(ty.clone());
        }
        
        // 不允许转换为对象
        if let Some(Object(_)) = res_type {
            return Err(CompileError::new(pos, "Cannot convert a value to an object by using 'as'.".to_string()));
        }
        
        // 保存当前表达式信息
        let this_ptr = this as *const Expr;
        self.expr_ope_type.insert(this_ptr, ope_type.clone());
        self.expr_res_type.insert(this_ptr, res_type.as_ref().unwrap().clone());
        return Ok(ExprResolveRes::new(res_type.unwrap(), ope_type));
    }
}
