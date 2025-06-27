//! Loxinas 函数模块

pub mod builtin_functions;

use crate::byte_handler::byte_writer::{write_dword, write_word};
use crate::instr::Instruction;
use crate::stmt::Stmt;
use crate::types::ValueType;

/// Loxinas 函数
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub enum LoxinasFunction {
    // 普通函数
    Normal {
        /// 函数符号
        symbol: String,
        /// 形参
        params: Vec<(String, ValueType)>,
        /// 返回类型
        return_type: ValueType,
        /// 函数表索引
        idx: usize,
        /// 代码
        chunk: *const Vec<Stmt>,
    },
    // 内置函数
    Builtin {
        /// 函数符号
        symbol: String,
        /// 形参
        params: Vec<(String, ValueType)>,
        /// 返回类型
        return_type: ValueType,
        /// 对应的内置函数
        builtin_function: fn () -> Vec<u8>,
    }
}

impl LoxinasFunction {
    pub fn get_symbol(&self) -> &String {
        match self {
            Self::Normal { symbol, .. } => symbol,
            Self::Builtin { symbol, .. } => symbol,
        }
    }
    
    pub fn get_params(&self) -> &[(String, ValueType)] {
        match self {
            Self::Normal { params, .. } => params,
            Self::Builtin { params, .. } => params,
        }
    }
    
    pub fn get_return_type(&self) -> &ValueType {
        match self {
            Self::Normal { return_type, .. } => return_type,
            Self::Builtin { return_type, .. } => return_type,
        }
    }
    
    pub fn call(&self, arg_size: u16) -> Vec<u8> {
        match self {
            Self::Normal { idx, .. } => {
                let mut code = vec![Instruction::OpPushWord.into()];
                write_word(&mut code, arg_size.to_le_bytes());
                code.push(Instruction::OpCall.into());
                write_dword(&mut code, (*idx as u32).to_le_bytes());
                code
            }
            Self::Builtin { builtin_function, .. } => builtin_function(),
        }
    }
}
