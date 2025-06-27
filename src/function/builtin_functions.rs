//! Loxinas 内置函数

use crate::instr::Instruction::*;
use crate::instr::SpecialFunction;

pub fn builtin_print_bool() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintBool.into()]
}

pub fn builtin_print_char() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintChar.into()]
}

pub fn builtin_print_byte() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintByte.into()]
}

pub fn builtin_print_sbyte() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintSByte.into()]
}

pub fn builtin_print_short() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintShort.into()]
}

pub fn builtin_print_ushort() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintUShort.into()]
}

pub fn builtin_print_int() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintInt.into()]
}

pub fn builtin_print_uint() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintUInt.into()]
}

pub fn builtin_print_long() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintLong.into()]
}

pub fn builtin_print_ulong() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintULong.into()]
}

pub fn builtin_print_extint() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintExtInt.into()]
}

pub fn builtin_print_uextint() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintUExtInt.into()]
}

pub fn builtin_print_float() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintFloat.into()]
}

pub fn builtin_print_double() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintDouble.into()]
}

pub fn builtin_println() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintNewLine.into()]
}

pub fn builtin_println_bool() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintBool.into(),
         OpSpecialFunction.into(), SpecialFunction::PrintNewLine.into()]
}

pub fn builtin_println_char() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintChar.into(),
         OpSpecialFunction.into(), SpecialFunction::PrintNewLine.into()]
}

pub fn builtin_println_byte() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintByte.into(),
         OpSpecialFunction.into(), SpecialFunction::PrintNewLine.into()]
}

pub fn builtin_println_sbyte() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintSByte.into(),
         OpSpecialFunction.into(), SpecialFunction::PrintNewLine.into()]
}

pub fn builtin_println_short() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintShort.into(),
         OpSpecialFunction.into(), SpecialFunction::PrintNewLine.into()]
}

pub fn builtin_println_ushort() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintUShort.into(),
         OpSpecialFunction.into(), SpecialFunction::PrintNewLine.into()]
}

pub fn builtin_println_int() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintInt.into(),
         OpSpecialFunction.into(), SpecialFunction::PrintNewLine.into()]
}

pub fn builtin_println_uint() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintUInt.into(),
         OpSpecialFunction.into(), SpecialFunction::PrintNewLine.into()]
}

pub fn builtin_println_long() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintLong.into(),
         OpSpecialFunction.into(), SpecialFunction::PrintNewLine.into()]
}

pub fn builtin_println_ulong() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintULong.into(),
         OpSpecialFunction.into(), SpecialFunction::PrintNewLine.into()]
}

pub fn builtin_println_extint() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintExtInt.into(),
         OpSpecialFunction.into(), SpecialFunction::PrintNewLine.into()]
}

pub fn builtin_println_uextint() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintUExtInt.into(),
         OpSpecialFunction.into(), SpecialFunction::PrintNewLine.into()]
}

pub fn builtin_println_float() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintFloat.into(),
         OpSpecialFunction.into(), SpecialFunction::PrintNewLine.into()]
}

pub fn builtin_println_double() -> Vec<u8> {
    vec![OpSpecialFunction.into(), SpecialFunction::PrintDouble.into(),
         OpSpecialFunction.into(), SpecialFunction::PrintNewLine.into()]
}
