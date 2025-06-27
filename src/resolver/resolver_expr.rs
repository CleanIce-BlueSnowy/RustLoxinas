//! 语义分析——表达式分析模块

use crate::data::{Data, DataFloat, DataInteger};
use crate::errors::error_types::{CompileError, CompileResult};
use crate::expr::{ExprAs, ExprBinary, ExprLiteral, ExprUnary, ExprVariable};
use crate::global_compiler::GlobalCompiler;
use crate::object::LoxinasClass;
use crate::resolver::{ExprResolveRes, Resolver};
use crate::tokens::{TokenKeyword, TokenOperator, TokenType};
use crate::types::ValueType::*;
use crate::types::{ValueFloatType, ValueType};

impl Resolver {
    /// 分析二元运算表达式
    pub fn resolve_binary_expr(
        &mut self,
        expr: &ExprBinary,
        left_res: &ExprResolveRes,
        right_res: &ExprResolveRes,
    ) -> CompileResult<ExprResolveRes> {
        let res_type;
        let ope_type;

        // 类型检查
        match (&left_res.res_type, &right_res.res_type) {
            // 两个字符，可以使用 `+` 将其合并为字符串，可以比较
            (Char, Char) => {
                use crate::tokens::TokenOperator::*;

                ope_type = Char;
                if let TokenType::Operator(Plus) = &expr.operator.token_type {
                    res_type = Object(LoxinasClass::String);
                } else if let TokenType::Operator(
                    EqualEqual | NotEqual | Less | LessEqual | Greater | GreaterEqual,
                ) = &expr.operator.token_type
                {
                    res_type = Bool;
                } else {
                    return Err(CompileError::new(
                        &expr.pos,
                        format!(
                            "Cannot ues operator '{}' between chars.",
                            Self::operator_to_string(&expr.operator)
                        ),
                    ));
                }
            }
            // 两个整数，运算时需要整型提升，操作符不能是布尔运算符；如果是位移运算符，则为左操作数类型
            (Integer(left_type), Integer(right_type)) => {
                use crate::tokens::TokenKeyword::*;
                use crate::types::ValueIntegerType::*;

                if let TokenType::Keyword(And | Or | Not) = &expr.operator.token_type {
                    return Err(CompileError::new(
                        &expr.pos,
                        format!(
                            "Cannot use operator '{}' between integers.",
                            Self::operator_to_string(&expr.operator)
                        ),
                    ));
                } else if let TokenType::Keyword(Shl | Shr) = &expr.operator.token_type {
                    if let Byte = right_type {
                        ope_type = Integer(left_type.clone());
                        res_type = Integer(left_type.clone());
                    } else {
                        return Err(CompileError::new(
                            &expr.pos,
                            "Must use 'byte' type in the right operation.".to_string(),
                        ));
                    }
                } else {
                    use crate::tokens::TokenOperator::*;

                    ope_type = match (left_type, right_type) {
                        (SByte, SByte) => Integer(SByte),

                        (SByte, Short) | (Short, Short) | (Short, SByte) => Integer(Short),

                        (SByte, Int) | (Short, Int) | (Int, Int) | (Int, SByte) | (Int, Short) => {
                            Integer(Int)
                        }

                        (SByte, Long)
                        | (Short, Long)
                        | (Int, Long)
                        | (Long, Long)
                        | (Long, Int)
                        | (Long, Short)
                        | (Long, SByte) => Integer(Long),

                        (Byte, ExtInt)
                        | (Short, ExtInt)
                        | (Int, ExtInt)
                        | (Long, ExtInt)
                        | (ExtInt, ExtInt)
                        | (ExtInt, Byte)
                        | (ExtInt, Short)
                        | (ExtInt, Int)
                        | (ExtInt, Long) => Integer(ExtInt),

                        (Byte, Byte) => Integer(Byte),

                        (Byte, UShort) | (UShort, UShort) | (UShort, Byte) => Integer(UShort),

                        (Byte, UInt)
                        | (UShort, UInt)
                        | (UInt, UInt)
                        | (UInt, Byte)
                        | (UInt, UShort) => Integer(UInt),

                        (Byte, ULong)
                        | (UShort, ULong)
                        | (UInt, ULong)
                        | (ULong, ULong)
                        | (ULong, UInt)
                        | (ULong, UShort)
                        | (ULong, Byte) => Integer(ULong),

                        (Byte, UExtInt)
                        | (UShort, UExtInt)
                        | (UInt, UExtInt)
                        | (ULong, UExtInt)
                        | (UExtInt, UExtInt)
                        | (UExtInt, Byte)
                        | (UExtInt, UShort)
                        | (UExtInt, UInt)
                        | (UExtInt, ULong) => Integer(UExtInt),

                        _ => {
                            return Err(CompileError::new(
                                &expr.pos,
                                "Cannot operate on two integers with different signs.".to_string(),
                            ))
                        }
                    };
                    res_type = if let TokenType::Operator(
                        EqualEqual | NotEqual | Less | LessEqual | Greater | GreaterEqual,
                    ) = &expr.operator.token_type
                    {
                        Bool
                    } else {
                        ope_type.clone()
                    };
                }
            }
            // 两个数字，其中一个是浮点数，结果提升为浮点数，操作符不能是布尔运算符、取模运算符以及位运算符
            (Integer(_), Float(float)) | (Float(float), Integer(_)) => {
                use crate::tokens::TokenKeyword::*;
                use crate::tokens::TokenOperator::*;

                ope_type = Float(float.clone());
                if let TokenType::Keyword(TokenKeyword::And | Or) = &expr.operator.token_type {
                    return Err(CompileError::new(
                        &expr.pos,
                        "Cannot use operator '{}' between numbers.".to_string(),
                    ));
                } else if let TokenType::Operator(TokenOperator::And | Pipe | Caret) =
                    &expr.operator.token_type
                {
                    return Err(CompileError::new(
                        &expr.pos,
                        "Cannot use operator '{}' between floating-point numbers.".to_string(),
                    ));
                } else if let TokenType::Operator(
                    EqualEqual | NotEqual | Less | LessEqual | Greater | GreaterEqual,
                ) = &expr.operator.token_type
                {
                    res_type = Bool;
                } else if let TokenType::Operator(Mod) = &expr.operator.token_type {
                    return Err(CompileError::new(
                        &expr.pos,
                        "Cannot use operator '%' on a floating-point number.".to_string(),
                    ));
                } else {
                    res_type = Float(float.clone());
                }
            }
            // 两个浮点数，需要提升，操作符不能是布尔运算符、取模运算符和位运算符
            (Float(left_type), Float(right_type)) => {
                use crate::tokens::TokenKeyword::*;
                use crate::tokens::TokenOperator::*;
                use crate::types::ValueFloatType::*;

                ope_type = ValueType::Float(match (left_type, right_type) {
                    (Float, Float) => Float,
                    _ => Double,
                });

                if let TokenType::Keyword(TokenKeyword::And | Or) = &expr.operator.token_type {
                    return Err(CompileError::new(
                        &expr.pos,
                        "Cannot use operator '{}' between floating-point numbers.".to_string(),
                    ));
                } else if let TokenType::Operator(TokenOperator::And | Pipe | Caret) =
                    &expr.operator.token_type
                {
                    return Err(CompileError::new(
                        &expr.pos,
                        "Cannot use operator '{}' between floating-point numbers.".to_string(),
                    ));
                } else if let TokenType::Operator(
                    EqualEqual | NotEqual | Less | LessEqual | Greater | GreaterEqual,
                ) = &expr.operator.token_type
                {
                    res_type = Bool;
                } else if let TokenType::Operator(Mod) = &expr.operator.token_type {
                    return Err(CompileError::new(
                        &expr.pos,
                        "Cannot use operator '%' between floating-point numbers.".to_string(),
                    ));
                } else {
                    res_type = ope_type.clone();
                }
            }
            // 两个布尔型，只支持布尔运算符、判等和不等号
            (Bool, Bool) => {
                use crate::tokens::TokenType::*;

                ope_type = Bool;
                match &expr.operator.token_type {
                    Operator(ope) => {
                        use crate::tokens::TokenOperator::*;

                        if let EqualEqual | NotEqual = ope {
                            res_type = Bool;
                        } else {
                            return Err(CompileError::new(
                                &expr.pos,
                                format!(
                                    "Cannot use operator '{}' between bools.",
                                    Self::operator_to_string(&expr.operator)
                                ),
                            ));
                        }
                    }
                    Keyword(ope) => {
                        use crate::tokens::TokenKeyword::*;

                        if let And | Or = ope {
                            res_type = Bool;
                        } else {
                            return Err(CompileError::new(
                                &expr.pos,
                                format!(
                                    "Cannot use operator '{}' between bools.",
                                    Self::operator_to_string(&expr.operator)
                                ),
                            ));
                        }
                    }
                    _ => unreachable!("Invalid operator"),
                }
            }
            // 两个对象，如果是字符串则可以拼接或比较，否则无效
            (Object(left_type), Object(right_type)) => {
                ope_type = Object(LoxinasClass::Object);

                if let (LoxinasClass::String, LoxinasClass::String) = (left_type, right_type) {
                    use crate::tokens::TokenType::*;

                    match &expr.operator.token_type {
                        Operator(ope) => {
                            use crate::tokens::TokenOperator::*;

                            if let Plus = ope {
                                res_type = Object(LoxinasClass::String);
                            } else if let EqualEqual | NotEqual | Less | LessEqual | Greater
                            | GreaterEqual = ope
                            {
                                res_type = Bool;
                            } else {
                                return Err(CompileError::new(
                                    &expr.pos,
                                    format!(
                                        "Cannot use operator '{}' between strings.",
                                        Self::operator_to_string(&expr.operator)
                                    ),
                                ));
                            }
                        }
                        _ => {
                            return Err(CompileError::new(
                                &expr.pos,
                                format!(
                                    "Cannot use operator '{}' between strings.",
                                    Self::operator_to_string(&expr.operator)
                                ),
                            ))
                        }
                    }
                } else {
                    return Err(CompileError::new(
                        &expr.pos,
                        "Cannot operate on two objects.".to_string(),
                    ));
                }
            }
            // 其余组合均无效
            (left_type, right_type) => {
                return Err(CompileError::new(
                    &expr.pos,
                    format!(
                        "Cannot use operator '{}' between '{}' and '{}'",
                        Self::operator_to_string(&expr.operator),
                        left_type,
                        right_type
                    ),
                ))
            }
        }

        Ok(ExprResolveRes::new(res_type, ope_type))
    }

    /// 分析分组表达式
    pub fn resolve_grouping_expr(
        &mut self,
        inside_expr_res: &ExprResolveRes,
    ) -> CompileResult<ExprResolveRes> {
        Ok(inside_expr_res.clone())
    }

    /// 分析字面量表达式
    pub fn resolve_literal_expr(&mut self, expr: &ExprLiteral) -> CompileResult<ExprResolveRes> {
        // 直接返回值的类型即可
        let ope_type = match &expr.value {
            Data::Char(_) => Char,
            Data::Bool(_) => Bool,
            Data::Float(float) => match float {
                DataFloat::Float(_) => Float(ValueFloatType::Float),
                DataFloat::Double(_) => Float(ValueFloatType::Double),
            },
            Data::Integer(integer) => Integer({
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
            }),
            Data::String(_) => Object(LoxinasClass::String),
        };

        let res_type = ope_type.clone();

        Ok(ExprResolveRes::new(res_type, ope_type))
    }

    /// 分析单元运算表达式
    pub fn resolve_unary_expr(
        &mut self,
        expr: &ExprUnary,
        right_res: &ExprResolveRes,
    ) -> CompileResult<ExprResolveRes> {
        let res_type;
        let ope_type;

        match &right_res.res_type {
            // 整数，结果为原类型，不允许布尔运算符，无符号整数不允许相反数（补码）
            Integer(integer) => {
                if let TokenType::Keyword(TokenKeyword::Not) = &expr.operator.token_type {
                    return Err(CompileError::new(
                        &expr.pos,
                        format!(
                            "Cannot use operator '{}' on an integer.",
                            Self::operator_to_string(&expr.operator)
                        ),
                    ));
                } else {
                    use crate::types::ValueIntegerType::*;

                    if let Byte | UShort | UInt | ULong | UExtInt = integer {
                        if let TokenType::Operator(TokenOperator::Minus) = &expr.operator.token_type
                        {
                            return Err(CompileError::new(
                                &expr.pos,
                                "Cannot use operator '-' on an unsigned integer.".to_string(),
                            ));
                        }
                    }

                    ope_type = Integer(integer.clone());
                }
            }
            // 浮点数，结果为原类型，不允许布尔运算符和位运算符
            Float(float) => {
                if let TokenType::Keyword(TokenKeyword::Not)
                | TokenType::Operator(TokenOperator::Tilde) = &expr.operator.token_type
                {
                    return Err(CompileError::new(
                        &expr.pos,
                        format!(
                            "Cannot use operator '{}' on a floating-point number.",
                            Self::operator_to_string(&expr.operator)
                        ),
                    ));
                } else {
                    ope_type = Float(float.clone())
                }
            }
            // 布尔型，结果为布尔型，只允许逻辑非运算符
            Bool => {
                if let TokenType::Keyword(TokenKeyword::Not) = &expr.operator.token_type {
                    ope_type = Bool;
                } else {
                    return Err(CompileError::new(
                        &expr.pos,
                        format!(
                            "Cannot use operator '{}' on a bool.",
                            Self::operator_to_string(&expr.operator)
                        ),
                    ));
                }
            }
            // 其他类型均无效
            expr_type => {
                return Err(CompileError::new(
                    &expr.pos,
                    format!(
                        "Cannot use '{}' on a '{}'",
                        Self::operator_to_string(&expr.operator),
                        expr_type
                    ),
                ))
            }
        }
        res_type = ope_type.clone();

        Ok(ExprResolveRes::new(res_type, ope_type))
    }

    /// 分析类型转换表达式
    pub fn resolve_as_expr(
        &mut self,
        expr: &ExprAs,
        inside_expr_res: &ExprResolveRes,
    ) -> CompileResult<ExprResolveRes> {
        let ope_type = &inside_expr_res.res_type;

        // 查找类型
        let res_type = GlobalCompiler::parse_value_type(&self.global_types, &expr.target)?;

        // 不允许在对象上使用 `as`
        if !Self::check_type_convert(ope_type, &res_type) {
            return Err(CompileError::new(
                &expr.pos,
                format!(
                    "Cannot use 'as' to convert '{}' to '{}'.",
                    ope_type, res_type
                ),
            ));
        }

        Ok(ExprResolveRes::new(res_type, ope_type.clone()))
    }

    pub fn resolve_variable_expr(
        &mut self,
        expr: &ExprVariable,
    ) -> CompileResult<(ExprResolveRes, usize, bool)> {
        // 获取变量
        let variable = if let Some(var) = self.find_variable(&expr.name) {
            var
        } else {
            return Err(CompileError::new(
                &expr.pos,
                "Undefined variable.".to_string(),
            ));
        };

        // 检查定义
        if !variable.defined {
            return Err(CompileError::new(
                &expr.pos,
                "Use a variable before it's defined.".to_string(),
            ));
        }

        // 检查初始化
        if !variable.initialized {
            return Err(CompileError::new(
                &expr.pos,
                "Use a variable before it's initialized.".to_string(),
            ));
        }

        // 设置类型
        let ty = variable.var_type.clone().unwrap();

        Ok((
            ExprResolveRes::new(ty.clone(), ty.clone()),
            variable.slot,
            variable.is_ref,
        ))
    }
}
