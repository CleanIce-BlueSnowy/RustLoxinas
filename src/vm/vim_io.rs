//! 虚拟机输入输出模块

use crate::vm::VM;

macro_rules! stdout_debug_info {
    () => {
        #[cfg(debug_assertions)]
        {
            print!(" <-- DEBUG MODE PROGRAM STDOUT --> ");
        }
    }
}

impl<'a> VM<'a> {
    /// 标准输出打印无符号字节
    #[inline]
    pub fn stdout_print_byte(byte: u8) {
        stdout_debug_info!();
        print!("{}", byte);
    }

    /// 标准输出打印有符号字节
    #[inline]
    pub fn stdout_print_sbyte(sbyte: i8) {
        stdout_debug_info!();
        print!("{}", sbyte);
    }

    /// 标准输出打印有符号短整型
    #[inline]
    pub fn stdout_print_short(short: i16) {
        stdout_debug_info!();
        print!("{}", short);
    }

    /// 标准输出打印无符号短整型
    #[inline]
    pub fn stdout_print_ushort(ushort: u16) {
        stdout_debug_info!();
        print!("{}", ushort);
    }

    /// 标准输出打印有符号整型
    #[inline]
    pub fn stdout_print_int(int: i32) {
        stdout_debug_info!();
        print!("{}", int);
    }

    /// 标准输出打印无符号整型
    #[inline]
    pub fn stdout_print_uint(uint: u32) {
        stdout_debug_info!();
        print!("{}", uint);
    }

    /// 标准输出打印有符号长整型
    #[inline]
    pub fn stdout_print_long(long: i64) {
        stdout_debug_info!();
        print!("{}", long);
    }

    /// 标准输出打印无符号长整型
    #[inline]
    pub fn stdout_print_ulong(ulong: u64) {
        stdout_debug_info!();
        print!("{}", ulong);
    }

    /// 标准输出打印有符号扩展整数
    #[inline]
    pub fn stdout_print_extint(extint: i128) {
        stdout_debug_info!();
        print!("{}", extint);
    }

    /// 标准输出打印无符号扩展整数
    #[inline]
    pub fn stdout_print_uextint(uextint: u128) {
        stdout_debug_info!();
        print!("{}", uextint);
    }

    /// 标准输出打印单精度浮点型
    #[inline]
    pub fn stdout_print_float(float: f32) {
        stdout_debug_info!();
        print!("{}", float);
    }

    /// 标准输出打印双精度浮点型
    #[inline]
    pub fn stdout_print_double(double: f64) {
        stdout_debug_info!();
        print!("{}", double);
    }

    /// 标准输出打印布尔型
    #[inline]
    pub fn stdout_print_bool(value: bool) {
        stdout_debug_info!();
        print!("{}", if value { "true" } else { "false" });
    }
    
    /// 标准输出打印字符
    #[inline]
    pub fn stdout_print_char(ch: char) {
        stdout_debug_info!();
        print!("{}", ch);
    }

    /// 标准输出打印换行符
    #[inline]
    pub fn stdout_print_new_line() {
        stdout_debug_info!();
        println!();
    }
}
