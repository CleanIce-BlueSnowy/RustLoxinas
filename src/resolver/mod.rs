//! 语义分析模块

use std::collections::HashMap;
use std::rc::Rc;

use crate::expr::Expr;
use crate::hashmap;
use crate::object::LoxinasClass;
use crate::position::Position;
use crate::tokens::Token;
use crate::types::ValueType;

mod resolver_expr;

/// 语义分析器
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Resolver {
    pub expr_ope_type: HashMap<*const Expr, ValueType>,
    pub expr_res_type: HashMap<*const Expr, ValueType>,
    pub global_types: HashMap<String, ValueType>,
}

impl Resolver {
    pub fn new() -> Self {
        Self { expr_ope_type: HashMap::new(), expr_res_type: HashMap::new(), global_types: Self::init_types() }
    }
    
    fn init_types() -> HashMap<String, ValueType> {
        use crate::types::ValueIntegerType::*;
        use crate::types::ValueFloatType::*;
        hashmap!{
            "char".to_string() => ValueType::Char,
            "bool".to_string() => ValueType::Bool,
            "byte".to_string() => ValueType::Integer(Byte),
            "sbyte".to_string() => ValueType::Integer(SByte),
            "short".to_string() => ValueType::Integer(Short),
            "ushort".to_string() => ValueType::Integer(UShort),
            "int".to_string() => ValueType::Integer(Int),
            "uint".to_string() => ValueType::Integer(UInt),
            "long".to_string() => ValueType::Integer(Long),
            "ulong".to_string() => ValueType::Integer(ULong),
            "eint".to_string() => ValueType::Integer(ExtInt),
            "ueint".to_string() => ValueType::Integer(UExtInt),
            "float".to_string() => ValueType::Float(Float),
            "double".to_string() => ValueType::Float(Double),
            "Object".to_string() => ValueType::Object(LoxinasClass::Object),
            "String".to_string() => ValueType::Object(LoxinasClass::String),
        }
    }
    
    /// 分析表达式
    pub fn resolve_expr(&mut self, expr: &Expr) -> Result<ExprResolveRes, CompileError> {
        expr.accept(self)
    }

    /// 操作符转字符串，方便报错
    pub fn operator_to_string(token: &Rc<Token>) -> String {
        use crate::tokens::TokenType;
        String::from(
            match &token.token_type {
                TokenType::Operator(operator) => {
                    use crate::tokens::TokenOperator::*;
                    match operator {
                        Plus => "+",
                        Minus => "-",
                        Star => "*",
                        Slash => "/",
                        Power => "**",
                        Backslash => "\\",
                        And => "&",
                        Pipe => "|",
                        Tilde => "~",
                        Colon => ":",
                        Semicolon => ";",
                        Equal => "=",
                        EqualEqual => "==",
                        NotEqual => "!=",
                        Less => "<",
                        LessEqual => "<=",
                        Greater => ">",
                        GreaterEqual => ">=",
                        Bang => "!",
                        Caret => "^",
                        Mod => "%",
                        DoubleColon => "::",
                    }
                }
                TokenType::Keyword(operator) => {
                    use crate::tokens::TokenKeyword::*;
                    match operator {
                        And => "and",
                        Or => "or",
                        Not => "not",
                        _ => panic!("Invalid operator"),
                    }
                }
                _ => panic!("Invalid operator"),
            }
        )
    }
}

/// 表达式语义分析结果
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ExprResolveRes {
    /// 表达式得到的结果类型
    pub res_type: ValueType,
    /// 操作时需要的数据类型
    pub ope_type: ValueType,
}

impl ExprResolveRes {
    pub fn new(expr_type: ValueType, ope_type: ValueType) -> Self {
        Self { res_type: expr_type, ope_type }
    }
}

/// 编译错误
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CompileError {
    /// 出错位置
    pub pos: Position,
    /// 错误信息
    pub message: String,
}

impl CompileError {
    pub fn new(pos: &Position, message: String) -> Self {
        Self { pos: pos.clone(), message }
    }
}
