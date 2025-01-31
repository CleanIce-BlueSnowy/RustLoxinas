//! 字节码指令模块

use num_enum::{IntoPrimitive, TryFromPrimitive};

/// 所有字节码指令
#[derive(IntoPrimitive, TryFromPrimitive, Clone)]
#[repr(u8)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Instruction {
    /// 返回
    OpReturn,
    /// 常数加载字节
    OpLoadConstByte,
    /// 常数加载单字
    OpLoadConstWord,
    /// 常数加载双字
    OpLoadConstDword,
    /// 常数加载四字
    OpLoadConstQword,
    /// 常数加载扩展整数（八字）
    OpLoadConstExtInt,
    /// 有符号位扩展，字节到单字
    OpSignExtendByteToWord,
    /// 有符号位扩展，单字到双字
    OpSignExtendWordToDword,
    /// 有符号位扩展，双字到四字
    OpSignExtendDwordToQword,
    /// 无符号位扩展，字节到单字
    OpZeroExtendByteToWord,
    /// 无符号位扩展，单字到双字
    OpZeroExtendWordToDword,
    /// 无符号位扩展，双字到四字
    OpZeroExtendDwordToQword,
    /// 位截断，四字到双字
    OpTruncateQwordToDword,
    /// 位截断，双字到单字
    OpTruncateDwordToWord,
    /// 位截断，单字到字节
    OpTruncateWordToByte,
    /// 字节加法
    OpIAddByte,
    /// 单字加法
    OpIAddWord,
    /// 双字加法
    OpIAddDword,
    /// 四字加法
    OpIAddQword,
    /// 字节减法
    OpISubByte,
    /// 单字减法
    OpISubWord,
    /// 双字减法
    OpISubDword,
    /// 四字减法
    OpISubQword,
    /// 字节乘法
    OpIMulByte,
    /// 单字乘法
    OpIMulWord,
    /// 双字乘法
    OpIMulDword,
    /// 四字乘法
    OpIMulQword,
    /// 字节有符号除法
    OpIDivSByte,
    /// 单字有符号除法
    OpIDivSWord,
    /// 双字有符号除法
    OpIDivSDword,
    /// 四字有符号除法
    OpIDivSQword,
    /// 字节无符号除法
    OpIDivUByte,
    /// 单字无符号除法
    OpIDivUWord,
    /// 双字无符号除法
    OpIDivUDword,
    /// 四字无符号除法
    OpIDivUQword,
    /// 字节有符号取模
    OpIModSByte,
    /// 单字有符号取模
    OpIModSWord,
    /// 双字有符号取模
    OpIModSDword,
    /// 四字有符号取模
    OpIModSQword,
    /// 字节无符号取模
    OpIModUByte,
    /// 单字无符号取模
    OpIModUWord,
    /// 双字无符号取模
    OpIModUDword,
    /// 四字无符号取模
    OpIModUQword,
    /// 字节相反数（补码）
    OpINegByte,
    /// 单字相反数（补码）
    OpINegWord,
    /// 双字相反数（补码）
    OpINegDword,
    /// 四字相反数（补码）
    OpINegQword,
    /// 有符号位扩展至扩展整数（八字）
    OpSignExtendToExtInt,
    /// 无符号位扩展至扩展整数（八字）
    OpZeroExtendToExtInt,
    /// 位截断从扩展整数（八字）
    OpTruncateFromExtInt,
    /// 扩展整数加法
    OpIAddExtInt,
    /// 扩展整数减法
    OpISubExtInt,
    /// 扩展整数乘法
    OpIMulExtInt,
    /// 扩展整数有符号除法
    OpIDivSExtInt,
    /// 扩展整数无符号除法
    OpIDivUExtInt,
    /// 扩展整数有符号取模
    OpIModSExtInt,
    /// 扩展整数无符号取模
    OpIModUExtInt,
    /// 扩展整数相反数（补码）
    OpINegExtInt,
    /// 有符号单字转化单精度浮点数
    OpConvertSWordToFloat,
    /// 无符号单字转化单精度浮点数
    OpConvertUWordToFloat,
    /// 有符号四字转化单精度浮点数
    OpConvertSQwordToFloat,
    /// 无符号四字转化单精度浮点数
    OpConvertUQwordToFloat,
    /// 有符号单字转化双精度浮点数
    OpConvertSWordToDouble,
    /// 无符号单字转化双精度浮点数
    OpConvertUWordToDouble,
    /// 有符号四字转化双精度浮点数
    OpConvertSQwordToDouble,
    /// 无符号四字转化双精度浮点数
    OpConvertUQwordToDouble,
    /// 单精度浮点数转化有符号单字
    OpConvertFloatToSWord,
    /// 单精度浮点数转化无符号单字
    OpConvertFloatToUWord,
    /// 单精度浮点数转化有符号四字
    OpConvertFloatToSQword,
    /// 单精度浮点数转化无符号四字
    OpConvertFloatToUQword,
    /// 双精度浮点数转化有符号单字
    OpConvertDoubleToSWord,
    /// 双精度浮点数转化无符号单字
    OpConvertDoubleToUWord,
    /// 双精度浮点数转化有符号四字
    OpConvertDoubleToSQword,
    /// 双精度浮点数转化无符号四字
    OpConvertDoubleToUQword,
    /// 有符号扩展整数转单精度浮点数
    OpConvertSExtIntToFloat,
    /// 无符号扩展整数转单精度浮点数
    OpConvertUExtIntToFloat,
    /// 有符号扩展整数转双精度浮点数
    OpConvertSExtIntToDouble,
    /// 无符号扩展整数转双精度浮点数
    OpConvertUExtIntToDouble,
    /// 单精度浮点数转有符号扩展整数
    OpConvertFloatToSExtInt,
    /// 单精度浮点数转无符号扩展整数
    OpConvertFloatToUExtInt,
    /// 双精度浮点数转有符号扩展整数
    OpConvertDoubleToSExtInt,
    /// 双精度浮点数转无符号扩展整数
    OpConvertDoubleToUExtInt,
    /// 单精度转双精度
    OpConvertFloatToDouble,
    /// 双精度转单精度
    OpConvertDoubleToFloat,
    /// 单精度加法
    OpFAddFloat,
    /// 双精度加法
    OpFAddDouble,
    /// 单精度减法
    OpFSubFloat,
    /// 双精度减法
    OpFSubDouble,
    /// 单精度乘法
    OpFMulFloat,
    /// 双精度乘法
    OpFMulDouble,
    /// 单精度除法
    OpFDivFloat,
    /// 双精度除法
    OpFDivDouble,
    /// 单精度相反数
    OpFNegFloat,
    /// 双精度相反数
    OpFNegDouble,
}
