//! 数据存储模块

/// 存储表达式解析和计算时的数据
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Data {
    Bool(bool),
    String(String),
    Integer(DataInteger),
    Float(DataFloat),
    Char(char),
}

/// 整型数据
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum DataInteger {
    Byte(u8),
    SByte(i8),
    Short(i16),
    UShort(u16),
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    ExtInt(i128),
    UExtInt(u128),
}

/// 浮点数据
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum DataFloat {
    Float(f32),
    Double(f64),
}

/// 数据大小
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum DataSize {
    Byte,
    Word,
    Dword,
    Qword,
    Oword,
}

impl DataSize {
    /// 获取字节数量
    #[inline]
    #[must_use]
    pub fn get_bytes_cnt(&self) -> usize {
        match self {
            DataSize::Byte => 1,
            DataSize::Word => 2,
            DataSize::Dword => 4,
            DataSize::Qword => 8,
            DataSize::Oword => 16,
        }
    }
}
