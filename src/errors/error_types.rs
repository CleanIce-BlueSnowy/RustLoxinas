//! 错误类型模块

use crate::position::Position;

/// 词法错误
#[cfg_attr(debug_assertions, derive(Debug))]
#[must_use]
pub struct LexicalError {
    pub pos: Position,
    pub message: String,
}

impl LexicalError {
    pub fn new(pos: &Position, message: String) -> Self {
        Self {
            pos: pos.clone(),
            message,
        }
    }
}

/// 语法错误
#[cfg_attr(debug_assertions, derive(Debug))]
#[must_use]
pub struct SyntaxError {
    pub pos: Position,
    pub message: String,
}

impl SyntaxError {
    pub fn new(pos: &Position, message: String) -> Self {
        Self {
            pos: pos.clone(),
            message,
        }
    }
}

/// 编译错误
#[cfg_attr(debug_assertions, derive(Debug))]
#[must_use]
pub struct CompileError {
    pub pos: Position,
    pub message: String,
}

impl CompileError {
    pub fn new(pos: &Position, message: String) -> Self {
        Self { 
            pos: pos.clone(), 
            message,
        }
    }
}

/// 运行时错误
#[must_use]
pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(message: String) -> Self {
        Self { 
            message,
        }
    }
}

pub type LexicalResult<Ret> = Result<Ret, LexicalError>;
pub type SyntaxResult<Ret> = Result<Ret, SyntaxError>;
pub type CompileResult<Ret> = Result<Ret, CompileError>;
pub type RuntimeResult<Ret> = Result<Ret, RuntimeError>;
pub type LexicalResultList<Ret> = Result<Ret, Vec<LexicalError>>;
pub type SyntaxResultList<Ret> = Result<Ret, Vec<SyntaxError>>;
pub type CompileResultList<Ret> = Result<Ret, Vec<CompileError>>;
