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
    /// 字节转布尔型
    OpConvertByteToBool,
    /// 单字转布尔型
    OpConvertWordToBool,
    /// 双字转布尔型
    OpConvertDwordToBool,
    /// 四字转布尔型
    OpConvertQwordToBool,
    /// 扩展整数转布尔型
    OpConvertExtIntToBool,
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
    /// 字节位取反
    OpBitNotByte,
    /// 单字位取反
    OpBitNotWord,
    /// 双字位取反
    OpBitNotDword,
    /// 四字位取反
    OpBitNotQword,
    /// 扩展整数位取反
    OpBitNotExtInt,
    /// 字节位与
    OpBitAndByte,
    /// 单字位与
    OpBitAndWord,
    /// 双字位与
    OpBitAndDword,
    /// 四字位与
    OpBitAndQword,
    /// 扩展整数位与
    OpBitAndExtInt,
    /// 字节位或
    OpBitOrByte,
    /// 单字位或
    OpBitOrWord,
    /// 双字位或
    OpBitOrDword,
    /// 四字位或
    OpBitOrQword,
    /// 扩展整数位或
    OpBitOrExtInt,
    /// 字节位异或
    OpBitXorByte,
    /// 单字位异或
    OpBitXorWord,
    /// 双字位异或
    OpBitXorDword,
    /// 四字位异或
    OpBitXorQword,
    /// 扩展整数位异或
    OpBitXorExtInt,
    /// 字节比较等于
    OpICmpEqualByte,
    /// 单字比较等于
    OpICmpEqualWord,
    /// 双字比较等于
    OpICmpEqualDword,
    /// 四字比较等于
    OpICmpEqualQword,
    /// 扩展整数比较等于
    OpICmpEqualExtInt,
    /// 字节比较不等于
    OpICmpNotEqualByte,
    /// 单字比较不等于
    OpICmpNotEqualWord,
    /// 双字比较不等于
    OpICmpNotEqualDword,
    /// 四字比较不等于
    OpICmpNotEqualQword,
    /// 扩展整数比较不等于
    OpICmpNotEqualExtInt,
    /// 有符号字节比较小于
    OpICmpLessSByte,
    /// 有符号单字比较小于
    OpICmpLessSWord,
    /// 有符号双字比较小于
    OpICmpLessSDword,
    /// 有符号四字比较小于
    OpICmpLessSQword,
    /// 有符号扩展整数比较小于
    OpICmpLessSExtInt,
    /// 无符号字节比较小于
    OpICmpLessUByte,
    /// 无符号单字比较小于
    OpICmpLessUWord,
    /// 无符号双字比较小于
    OpICmpLessUDword,
    /// 无符号四字比较小于
    OpICmpLessUQword,
    /// 无符号扩展整数比较小于
    OpICmpLessUExtInt,
    /// 有符号字节比较小于等于
    OpICmpLessEqualSByte,
    /// 有符号单字比较小于等于
    OpICmpLessEqualSWord,
    /// 有符号双字比较小于等于
    OpICmpLessEqualSDword,
    /// 有符号四字比较小于等于
    OpICmpLessEqualSQword,
    /// 有符号扩展整数比较小于等于
    OpICmpLessEqualSExtInt,
    /// 无符号字节比较小于等于
    OpICmpLessEqualUByte,
    /// 无符号单字比较小于等于
    OpICmpLessEqualUWord,
    /// 无符号双字比较小于等于
    OpICmpLessEqualUDword,
    /// 无符号四字比较小于等于
    OpICmpLessEqualUQword,
    /// 无符号扩展整数比较小于等于
    OpICmpLessEqualUExtInt,
    /// 有符号字节比较大于
    OpICmpGreaterSByte,
    /// 有符号单字比较大于
    OpICmpGreaterSWord,
    /// 有符号双字比较大于
    OpICmpGreaterSDword,
    /// 有符号四字比较大于
    OpICmpGreaterSQword,
    /// 有符号扩展整数比较大于
    OpICmpGreaterSExtInt,
    /// 无符号字节比较大于
    OpICmpGreaterUByte,
    /// 无符号单字比较大于
    OpICmpGreaterUWord,
    /// 无符号双字比较大于
    OpICmpGreaterUDword,
    /// 无符号四字比较大于
    OpICmpGreaterUQword,
    /// 无符号扩展整数比较大于
    OpICmpGreaterUExtInt,
    /// 有符号字节比较大于等于
    OpICmpGreaterEqualSByte,
    /// 有符号单字比较大于等于
    OpICmpGreaterEqualSWord,
    /// 有符号双字比较大于等于
    OpICmpGreaterEqualSDword,
    /// 有符号四字比较大于等于
    OpICmpGreaterEqualSQword,
    /// 有符号扩展整数比较大于等于
    OpICmpGreaterEqualSExtInt,
    /// 无符号字节比较大于等于
    OpICmpGreaterEqualUByte,
    /// 无符号单字比较大于等于
    OpICmpGreaterEqualUWord,
    /// 无符号双字比较大于等于
    OpICmpGreaterEqualUDword,
    /// 无符号四字比较大于等于
    OpICmpGreaterEqualUQword,
    /// 无符号扩展整数比较大于等于
    OpICmpGreaterEqualUExtInt,
    /// 单精度比较等于
    OpFCmpEqualFloat,
    /// 双精度比较等于
    OpFCmpEqualDouble,
    /// 单精度比较不等于
    OpFCmpNotEqualFloat,
    /// 双精度比较不等于
    OpFCmpNotEqualDouble,
    /// 单精度比较小于
    OpFCmpLessFloat,
    /// 双精度比较小于
    OpFCmpLessDouble,
    /// 单精度比较小于等于
    OpFCmpLessEqualFloat,
    /// 双精度比较小于等于
    OpFCmpLessEqualDouble,
    /// 单精度比较大于
    OpFCmpGreaterFloat,
    /// 双精度比较大于
    OpFCmpGreaterDouble,
    /// 单精度比较大于等于
    OpFCmpGreaterEqualFloat,
    /// 双精度比较大于等于
    OpFCmpGreaterEqualDouble,
    /// 弹出字节
    OpPopByte,
    /// 弹出单字
    OpPopWord,
    /// 弹出双字
    OpPopDword,
    /// 弹出四字
    OpPopQword,
    /// 弹出扩展整数
    OpPopExtInt,
    /// 压入字节
    OpPushByte,
    /// 压入单字
    OpPushWord,
    /// 压入双字
    OpPushDword,
    /// 压入四字
    OpPushQword,
    /// 压入扩展整数
    OpPushExtInt,
    /// 获取局部变量字节
    OpGetLocalByte,
    /// 获取局部变量单字
    OpGetLocalWord,
    /// 获取局部变量双字
    OpGetLocalDword,
    /// 获取局部变量四字
    OpGetLocalQword,
    /// 获取局部变量扩展整数
    OpGetLocalExtInt,
}
