//! 位置信息模块

/// 用于记录位置信息
#[derive(Debug, Clone)]
pub struct Position {
    /// 起始位置所在行
    pub start_line: usize,
    /// 起始位置在起始行的索引
    pub start_idx: usize,
    /// 终止位置所在行
    pub end_line: usize,
    /// 终止位置在终止行的索引
    pub end_idx: usize,
}

impl Position {
    pub fn new(start_line: usize, start_idx: usize, end_line: usize, end_idx: usize) -> Self {
        Self { start_line, start_idx, end_line, end_idx }
    }
}
