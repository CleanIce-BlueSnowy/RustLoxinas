//! 虚拟机输入输出模块

use crate::vm::VM;

impl<'a> VM<'a> {
    /// 标准输出打印无符号字节
    #[inline]
    pub fn stdout_print_byte(byte: u8) {
        #[cfg(debug_assertions)]
        {
            print!(" <--- DEBUG MODE PROGRAM STDOUT ---> ");
        }
        print!("{}", byte);
    }

    /// 标准输出打印有符号字节
    #[inline]
    pub fn stdout_print_sbyte(sbyte: i8) {
        #[cfg(debug_assertions)]
        {
            print!(" <--- DEBUG MODE PROGRAM STDOUT ---> ");
        }
        print!("{}", sbyte);
    }

    /// 标准输出打印有符号短整型
    #[inline]
    pub fn stdout_print_short(short: i16) {
        #[cfg(debug_assertions)]
        {
            print!(" <--- DEBUG MODE PROGRAM STDOUT ---> ");
        }
        print!("{}", short);
    }

    /// 标准输出打印无符号短整型
    #[inline]
    pub fn stdout_print_ushort(ushort: u16) {
        #[cfg(debug_assertions)]
        {
            print!(" <--- DEBUG MODE PROGRAM STDOUT ---> ");
        }
        print!("{}", ushort);
    }

    /// 标准输出打印有符号整型
    #[inline]
    pub fn stdout_print_int(int: i32) {
        #[cfg(debug_assertions)]
        {
            print!(" <--- DEBUG MODE PROGRAM STDOUT ---> ");
        }
        print!("{}", int);
    }

    /// 标准输出打印无符号整型
    #[inline]
    pub fn stdout_print_uint(uint: u32) {
        #[cfg(debug_assertions)]
        {
            print!(" <--- DEBUG MODE PROGRAM STDOUT ---> ");
        }
        print!("{}", uint);
    }

    /// 标准输出打印有符号长整型
    #[inline]
    pub fn stdout_print_long(long: i64) {
        #[cfg(debug_assertions)]
        {
            print!(" <--- DEBUG MODE PROGRAM STDOUT ---> ");
        }
        print!("{}", long);
    }

    /// 标准输出打印无符号长整型
    #[inline]
    pub fn stdout_print_ulong(ulong: u64) {
        #[cfg(debug_assertions)]
        {
            print!(" <--- DEBUG MODE PROGRAM STDOUT ---> ");
        }
        print!("{}", ulong);
    }

    /// 标准输出打印有符号扩展整数
    #[inline]
    pub fn stdout_print_extint(extint: i128) {
        #[cfg(debug_assertions)]
        {
            print!(" <--- DEBUG MODE PROGRAM STDOUT ---> ");
        }
        print!("{}", extint);
    }

    /// 标准输出打印无符号扩展整数
    #[inline]
    pub fn stdout_print_uextint(uextint: u128) {
        #[cfg(debug_assertions)]
        {
            print!(" <--- DEBUG MODE PROGRAM STDOUT ---> ");
        }
        print!("{}", uextint);
    }

    /// 标准输出打印单精度浮点型
    #[inline]
    pub fn stdout_print_float(float: f32) {
        #[cfg(debug_assertions)]
        {
            print!(" <--- DEBUG MODE PROGRAM STDOUT ---> ");
        }
        print!("{}", float);
    }

    /// 标准输出打印双精度浮点型
    #[inline]
    pub fn stdout_print_double(double: f64) {
        #[cfg(debug_assertions)]
        {
            print!(" <--- DEBUG MODE PROGRAM STDOUT ---> ");
        }
        print!("{}", double);
    }

    /// 标准输出打印布尔型
    #[inline]
    pub fn stdout_print_bool(value: bool) {
        #[cfg(debug_assertions)]
        {
            print!(" <--- DEBUG MODE PROGRAM STDOUT ---> ");
        }
        print!("{}", if value { "true" } else { "false" });
    }

    /// 标准输出打印换行符
    #[inline]
    pub fn stdout_print_new_line() {
        #[cfg(debug_assertions)]
        {
            print!(" <--- DEBUG MODE PROGRAM STDOUT ---> ");
        }
        println!();
    }
}
