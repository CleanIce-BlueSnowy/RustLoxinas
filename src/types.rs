//! 数据类型模块

use crate::data::DataSize;
use crate::object::LoxinasClass;
use crate::position::Position;
use std::collections::LinkedList;
use std::fmt::Display;

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

/// SAFETY: 用于 lazy_static
unsafe impl Sync for ValueType {}

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

impl ValueType {
    /// 获取该类型数据的数据大小
    #[must_use]
    pub fn get_size(&self) -> DataSize {
        use ValueFloatType::*;
        use ValueIntegerType::*;
        use ValueType::*;

        match self {
            Char => DataSize::Dword,
            Bool => DataSize::Byte,
            Integer(integer) => match integer {
                SByte | Byte => DataSize::Byte,
                Short | UShort => DataSize::Word,
                Int | UInt => DataSize::Dword,
                Long | ULong => DataSize::Qword,
                ExtInt | UExtInt => DataSize::Oword,
            },
            ValueType::Float(float) => match float {
                ValueFloatType::Float => DataSize::Dword,
                Double => DataSize::Qword,
            },
            Object(_) => DataSize::Qword,
        }
    }
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
                    ExtInt => "extint",
                    UExtInt => "uextint",
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
        }
        .to_string();

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
    #[must_use]
    pub fn new() -> Self {
        Self {
            pos: Position::new(0, 0, 0, 0),
            chain: LinkedList::new(),
        }
    }
}

impl Display for TypeTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        let mut first_type = true;

        for type_name in &self.chain {
            res.push_str(if first_type {
                first_type = false;
                ""
            } else {
                "::"
            });
            res.push_str(&type_name);
        }

        return write!(f, "{}", res);
    }
}
