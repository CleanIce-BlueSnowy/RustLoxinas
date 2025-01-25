use std::fmt::Display;
use crate::object::LoxinasClass;

/// 语义分析使用的数据类型
#[cfg_attr(debug_assertions, derive(Debug))]
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

/// 整数类型
#[cfg_attr(debug_assertions, derive(Debug))]
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
            Object(_) => "object",
        }.to_string();
        return write!(formatter, "{}", str);
    }
}
