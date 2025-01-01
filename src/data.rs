/// 存储表达式解析和计算时的数据
#[allow(dead_code)]
#[derive(Debug)]
pub enum Data {
    Bool(bool),
    String(String),
    Integer(DataInteger),
    Float(DataFloat),
}

/// 整型数据
#[allow(dead_code)]
#[derive(Debug)]
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
#[allow(dead_code)]
#[derive(Debug)]
pub enum DataFloat {
    Float(f32),
    Double(f64),
}
