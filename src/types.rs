//! 数据类型模块

use std::collections::LinkedList;
use std::fmt::Display;

use crate::object::LoxinasClass;
use crate::position::Position;

/// 数据类型
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub enum ValueType {
    /// 整数类型
    Integer(ValueIntegerType),
    /// 浮点类型
    Float(ValueFloatType),
    /// 字符型
    Char,
    /// 布尔型
    Bool,
    /// 引用类型（对象）
    Object(LoxinasClass),
}

unsafe impl Sync for ValueType {
    
}

/// 整数类型
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub enum ValueIntegerType {
    Byte,
    SByte,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    ExtInt,
    UExtInt,
}

/// 浮点类型
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub enum ValueFloatType {
    Float,
    Double,
}

impl Display for ValueType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ValueType::*;
        let str = match self {
            Integer(integer) => {
                use ValueIntegerType::*;
                match integer {
                    Byte => "byte",
                    SByte => "sbyte",
                    Short => "short",
                    UShort => "ushort",
                    Int => "int",
                    UInt => "uint",
                    Long => "long",
                    ULong => "ulong",
                    ExtInt => "eint",
                    UExtInt => "ueint",
                }
            }
            Float(float) => {
                use ValueFloatType::*;
                match float {
                    Float => "float",
                    Double => "double",
                }
            }
            Char => "char",
            Bool => "bool",
            Object(object) => object.get_name(),
        }.to_string();
        return write!(formatter, "{}", str);
    }
}

/// 类型标签
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TypeTag {
    /// 位置信息
    pub pos: Position,
    /// 类型链
    pub chain: LinkedList<String>,
}

impl TypeTag {
    pub fn new() -> Self {
        Self { pos: Position::new(0, 0, 0, 0), chain: LinkedList::new() }
    }
}
