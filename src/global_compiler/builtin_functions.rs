//! 全局编译器——内置函数模块

use std::collections::HashMap;
use crate::function::LoxinasFunction;
use crate::global_compiler::GlobalCompiler;
use crate::hashmap;
use crate::types::ValueFloatType;

impl GlobalCompiler {
    // 初始化内置函数
    pub fn init_builtin_functions() -> HashMap<String, Vec<LoxinasFunction>> {
        use crate::types::ValueType::*;
        use crate::types::ValueIntegerType::*;
        use crate::function::builtin_functions::*;

        hashmap! {
            "print".to_string() => vec![
                LoxinasFunction::Builtin {
                    symbol: "print#bool$unit".to_string(),
                    params: vec![("value".to_string(), Bool)],
                    return_type: Unit,
                    builtin_function: builtin_print_bool,
                },
                LoxinasFunction::Builtin {
                    symbol: "print#char$unit".to_string(),
                    params: vec![("value".to_string(), Char)],
                    return_type: Unit,
                    builtin_function: builtin_print_char,
                },
                LoxinasFunction::Builtin {
                    symbol: "print#byte$unit".to_string(),
                    params: vec![("value".to_string(), Integer(Byte))],
                    return_type: Unit,
                    builtin_function: builtin_print_byte,
                },
                LoxinasFunction::Builtin {
                    symbol: "print#sbyte$unit".to_string(),
                    params: vec![("value".to_string(), Integer(SByte))],
                    return_type: Unit,
                    builtin_function: builtin_print_sbyte,
                },
                LoxinasFunction::Builtin {
                    symbol: "print#short$unit".to_string(),
                    params: vec![("value".to_string(), Integer(Short))],
                    return_type: Unit,
                    builtin_function: builtin_print_short,
                },
                LoxinasFunction::Builtin {
                    symbol: "print#ushort$unit".to_string(),
                    params: vec![("value".to_string(), Integer(UShort))],
                    return_type: Unit,
                    builtin_function: builtin_print_ushort,
                },
                LoxinasFunction::Builtin {
                    symbol: "print#int$unit".to_string(),
                    params: vec![("value".to_string(), Integer(Int))],
                    return_type: Unit,
                    builtin_function: builtin_print_int,
                },
                LoxinasFunction::Builtin {
                    symbol: "print#uint$unit".to_string(),
                    params: vec![("value".to_string(), Integer(UInt))],
                    return_type: Unit,
                    builtin_function: builtin_print_uint,
                },
                LoxinasFunction::Builtin {
                    symbol: "print#long$unit".to_string(),
                    params: vec![("value".to_string(), Integer(Long))],
                    return_type: Unit,
                    builtin_function: builtin_print_long,
                },
                LoxinasFunction::Builtin {
                    symbol: "print#ulong$unit".to_string(),
                    params: vec![("value".to_string(), Integer(ULong))],
                    return_type: Unit,
                    builtin_function: builtin_print_ulong,
                },
                LoxinasFunction::Builtin {
                    symbol: "print#extint$unit".to_string(),
                    params: vec![("value".to_string(), Integer(ExtInt))],
                    return_type: Unit,
                    builtin_function: builtin_print_extint,
                },
                LoxinasFunction::Builtin {
                    symbol: "print#uextint$unit".to_string(),
                    params: vec![("value".to_string(), Integer(UExtInt))],
                    return_type: Unit,
                    builtin_function: builtin_print_uextint,
                },
                LoxinasFunction::Builtin {
                    symbol: "print#float$unit".to_string(),
                    params: vec![("value".to_string(), Float(ValueFloatType::Float))],
                    return_type: Unit,
                    builtin_function: builtin_print_float,
                },
                LoxinasFunction::Builtin {
                    symbol: "print#double$unit".to_string(),
                    params: vec![("value".to_string(), Float(ValueFloatType::Double))],
                    return_type: Unit,
                    builtin_function: builtin_print_double,
                },
            ],
            "println".to_string() => vec![
                LoxinasFunction::Builtin {
                    symbol: "println$unit".to_string(),
                    params: vec![],
                    return_type: Unit,
                    builtin_function: builtin_println,
                },
                LoxinasFunction::Builtin {
                    symbol: "println#bool$unit".to_string(),
                    params: vec![("value".to_string(), Bool)],
                    return_type: Unit,
                    builtin_function: builtin_println_bool,
                },
                LoxinasFunction::Builtin {
                    symbol: "println#char$unit".to_string(),
                    params: vec![("value".to_string(), Char)],
                    return_type: Unit,
                    builtin_function: builtin_println_char,
                },
                LoxinasFunction::Builtin {
                    symbol: "println#byte$unit".to_string(),
                    params: vec![("value".to_string(), Integer(Byte))],
                    return_type: Unit,
                    builtin_function: builtin_println_byte,
                },
                LoxinasFunction::Builtin {
                    symbol: "println#sbyte$unit".to_string(),
                    params: vec![("value".to_string(), Integer(SByte))],
                    return_type: Unit,
                    builtin_function: builtin_println_sbyte,
                },
                LoxinasFunction::Builtin {
                    symbol: "println#short$unit".to_string(),
                    params: vec![("value".to_string(), Integer(Short))],
                    return_type: Unit,
                    builtin_function: builtin_println_short,
                },
                LoxinasFunction::Builtin {
                    symbol: "println#ushort$unit".to_string(),
                    params: vec![("value".to_string(), Integer(UShort))],
                    return_type: Unit,
                    builtin_function: builtin_println_ushort,
                },
                LoxinasFunction::Builtin {
                    symbol: "println#int$unit".to_string(),
                    params: vec![("value".to_string(), Integer(Int))],
                    return_type: Unit,
                    builtin_function: builtin_println_int,
                },
                LoxinasFunction::Builtin {
                    symbol: "println#uint$unit".to_string(),
                    params: vec![("value".to_string(), Integer(UInt))],
                    return_type: Unit,
                    builtin_function: builtin_println_uint,
                },
                LoxinasFunction::Builtin {
                    symbol: "println#long$unit".to_string(),
                    params: vec![("value".to_string(), Integer(Long))],
                    return_type: Unit,
                    builtin_function: builtin_println_long,
                },
                LoxinasFunction::Builtin {
                    symbol: "println#ulong$unit".to_string(),
                    params: vec![("value".to_string(), Integer(ULong))],
                    return_type: Unit,
                    builtin_function: builtin_println_ulong,
                },
                LoxinasFunction::Builtin {
                    symbol: "println#extint$unit".to_string(),
                    params: vec![("value".to_string(), Integer(ExtInt))],
                    return_type: Unit,
                    builtin_function: builtin_println_extint,
                },
                LoxinasFunction::Builtin {
                    symbol: "println#uextint$unit".to_string(),
                    params: vec![("value".to_string(), Integer(UExtInt))],
                    return_type: Unit,
                    builtin_function: builtin_println_uextint,
                },
                LoxinasFunction::Builtin {
                    symbol: "println#float$unit".to_string(),
                    params: vec![("value".to_string(), Float(ValueFloatType::Float))],
                    return_type: Unit,
                    builtin_function: builtin_println_float,
                },
                LoxinasFunction::Builtin {
                    symbol: "println#double$unit".to_string(),
                    params: vec![("value".to_string(), Float(ValueFloatType::Double))],
                    return_type: Unit,
                    builtin_function: builtin_println_double,
                },
            ]
        }
    }
}
