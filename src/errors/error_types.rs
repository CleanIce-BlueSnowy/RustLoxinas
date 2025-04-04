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
        Self { pos: pos.clone(), message }
    }
}

/// 语法错误
#[cfg_attr(debug_assertions, derive(Debug))]
#[must_use]
pub struct SyntaxError {
    /// 错误位置
    pub pos: Position,
    /// 错误信息
    pub message: String,
}

impl SyntaxError {
    pub fn new(pos: &Position, message: String) -> Self {
        Self { pos: pos.clone(), message }
    }
}

/// 编译错误
#[cfg_attr(debug_assertions, derive(Debug))]
#[must_use]
pub struct CompileError {
    /// 出错位置
    pub pos: Position,
    /// 错误信息
    pub message: String,
}

impl CompileError {
    pub fn new(pos: &Position, message: String) -> Self {
        Self { pos: pos.clone(), message }
    }
}

/// 运行时错误
#[must_use]
pub struct RuntimeError {
    /// 错误信息
    pub message: String,
}

impl RuntimeError {
    pub fn new(msg: String) -> Self {
        Self { message: msg }
    }
}

pub type LexicalResult<Ret> = Result<Ret, LexicalError>;
pub type SyntaxResult<Ret> = Result<Ret, SyntaxError>;
pub type CompileResult<Ret> = Result<Ret, CompileError>;
pub type RuntimeResult<Ret> = Result<Ret, RuntimeError>;
