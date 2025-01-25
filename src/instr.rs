use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Instructions {
    /// 返回
    OpRet,
    /// 常数加载字节
    OpConstB,
    /// 常数加载单字
    OpConstW,
    /// 常数加载双字
    OpConstD,
    /// 常数加载四字
    OpConstQ,
    /// 有符号位扩展，字节到单字
    OpBitExtSBW,
    /// 有符号位扩展，单字到双字
    OpBitExtSWD,
    /// 有符号位扩展，双字到四字
    OpBitExtSDQ,
    /// 无符号位扩展，字节到单字
    OpBitExtUBW,
    /// 无符号位扩展，单字到双字
    OpBitExtUWD,
    /// 无符号位扩展，双字到四字
    OpBitExtUBQ,
    /// 位截断，四字到双字
    OpBitTruQD,
    /// 位截断，双字到单字
    OpBitTruDW,
    /// 位截断，单字到字节
    OpBitTruWB,
}
