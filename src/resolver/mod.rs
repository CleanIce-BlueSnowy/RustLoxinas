//! 语义分析模块

use std::collections::HashMap;
use std::rc::Rc;

use crate::hashmap;
use crate::object::LoxinasClass;
use crate::stmt::Stmt;
use crate::tokens::Token;
use crate::types::ValueType;

mod resolver_expr;
mod resolver_stmt;
mod resolver_assistance;

/// 语义分析器
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Resolver {
    pub global_types: HashMap<String, ValueType>,
    pub variables: Vec<(HashMap<String, Variable>, usize)>,
    pub now_slot: usize,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            global_types: Self::init_types(),
            variables: vec![],
            now_slot: 0,
        }
    }

    /// 初始化全局类型列表
    fn init_types() -> HashMap<String, ValueType> {
        use crate::types::ValueIntegerType::*;
        use crate::types::ValueFloatType::*;
        hashmap! {
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
            "extint".to_string() => ValueType::Integer(ExtInt),
            "uextint".to_string() => ValueType::Integer(UExtInt),
            "float".to_string() => ValueType::Float(Float),
            "double".to_string() => ValueType::Float(Double),
            "Object".to_string() => ValueType::Object(LoxinasClass::Object),
            "String".to_string() => ValueType::Object(LoxinasClass::String),
        }
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
                        Shl => "shl",
                        Shr => "shr",
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
#[derive(Clone)]
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

/// 变量
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct Variable {
    /// 定义位置的语句指针
    pub define_stmt: *const Stmt,
    /// 已定义
    pub defined: bool,
    /// 已初始化
    pub initialized: bool,
    /// 栈偏移量
    pub slot: usize,
    /// 变量类型
    pub var_type: Option<ValueType>,
    /// 是否为引用
    pub is_ref: bool,
}

impl Variable {
    pub fn new(define_stmt: *const Stmt, defined: bool, initialized: bool, slot: usize, var_type: Option<ValueType>, is_ref: bool) -> Self {
        Self { define_stmt, defined, initialized, slot, var_type, is_ref }
    }
}
