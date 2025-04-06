//! 字节码指令模块

use num_enum::{IntoPrimitive, TryFromPrimitive};

/// 所有字节码指令
#[derive(IntoPrimitive, TryFromPrimitive, Clone)]
#[repr(u8)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Instruction {
    /// 特殊功能
    OpSpecialFunction,
    /// 返回
    OpReturn,
    /// 无条件跳转
    OpJump,
    /// 为真时跳转
    OpJumpTrue,
    /// 为真时跳转并弹出数值
    OpJumpTruePop,
    /// 为假时跳转
    OpJumpFalse,
    /// 为假时跳转并弹出数值
    OpJumpFalsePop,
    /// 常数加载字节
    OpLoadConstByte,
    /// 常数加载单字
    OpLoadConstWord,
    /// 常数加载双字
    OpLoadConstDword,
    /// 常数加载四字
    OpLoadConstQword,
    /// 常数加载八字
    OpLoadConstOword,
    /// 有符号位扩展，字节到单字
    OpSignExtendByteToWord,
    /// 有符号位扩展，单字到双字
    OpSignExtendWordToDword,
    /// 有符号位扩展，双字到四字
    OpSignExtendDwordToQword,
    /// 有符号位扩展，四字到八字
    OpSignExtendQwordToOword,
    /// 无符号位扩展，字节到单字
    OpZeroExtendByteToWord,
    /// 无符号位扩展，单字到双字
    OpZeroExtendWordToDword,
    /// 无符号位扩展，双字到四字
    OpZeroExtendDwordToQword,
    /// 无符号位扩展，四字到八字
    OpZeroExtendQwordToOword,
    /// 位截断，八字到四字
    OpTruncateOwordToQword,
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
    /// 八字加法
    OpIAddOword,
    /// 字节减法
    OpISubByte,
    /// 单字减法
    OpISubWord,
    /// 双字减法
    OpISubDword,
    /// 四字减法
    OpISubQword,
    /// 八字减法
    OpISubOword,
    /// 字节乘法
    OpIMulByte,
    /// 单字乘法
    OpIMulWord,
    /// 双字乘法
    OpIMulDword,
    /// 四字乘法
    OpIMulQword,
    /// 八字乘法
    OpIMulOword,
    /// 字节有符号除法
    OpIDivSByte,
    /// 单字有符号除法
    OpIDivSWord,
    /// 双字有符号除法
    OpIDivSDword,
    /// 四字有符号除法
    OpIDivSQword,
    /// 八字有符号除法
    OpIDivSOword,
    /// 字节无符号除法
    OpIDivUByte,
    /// 单字无符号除法
    OpIDivUWord,
    /// 双字无符号除法
    OpIDivUDword,
    /// 四字无符号除法
    OpIDivUQword,
    /// 八字无符号除法
    OpIDivUOword,
    /// 字节有符号取模
    OpIModSByte,
    /// 单字有符号取模
    OpIModSWord,
    /// 双字有符号取模
    OpIModSDword,
    /// 四字有符号取模
    OpIModSQword,
    /// 八字有符号取模
    OpIModSOword,
    /// 字节无符号取模
    OpIModUByte,
    /// 单字无符号取模
    OpIModUWord,
    /// 双字无符号取模
    OpIModUDword,
    /// 四字无符号取模
    OpIModUQword,
    /// 八字无符号取模
    OpIModUOword,
    /// 字节相反数（补码）
    OpINegByte,
    /// 单字相反数（补码）
    OpINegWord,
    /// 双字相反数（补码）
    OpINegDword,
    /// 四字相反数（补码）
    OpINegQword,
    /// 八字相反数（补码）
    OpINegOword,
    /// 有符号单字转单精度浮点数
    OpConvertSWordToFloat,
    /// 无符号单字转单精度浮点数
    OpConvertUWordToFloat,
    /// 有符号四字转单精度浮点数
    OpConvertSQwordToFloat,
    /// 无符号四字转单精度浮点数
    OpConvertUQwordToFloat,
    /// 有符号八字转单精度浮点数
    OpConvertSOwordToFloat,
    /// 无符号八字转单精度浮点数
    OpConvertUOwordToFloat,
    /// 有符号单字转双精度浮点数
    OpConvertSWordToDouble,
    /// 无符号单字转双精度浮点数
    OpConvertUWordToDouble,
    /// 有符号四字转双精度浮点数
    OpConvertSQwordToDouble,
    /// 无符号四字转双精度浮点数
    OpConvertUQwordToDouble,
    /// 有符号八字转双精度浮点数
    OpConvertSOwordToDouble,
    /// 无符号八字转双精度浮点数
    OpConvertUOwordToDouble,
    /// 单精度浮点数转有符号单字
    OpConvertFloatToSWord,
    /// 单精度浮点数转无符号单字
    OpConvertFloatToUWord,
    /// 单精度浮点数转有符号四字
    OpConvertFloatToSQword,
    /// 单精度浮点数转无符号四字
    OpConvertFloatToUQword,
    /// 单精度浮点数转有符号八字
    OpConvertFloatToSOword,
    /// 单精度浮点数转无符号八字
    OpConvertFloatToUOword,
    /// 双精度浮点数转有符号单字
    OpConvertDoubleToSWord,
    /// 双精度浮点数转无符号单字
    OpConvertDoubleToUWord,
    /// 双精度浮点数转有符号四字
    OpConvertDoubleToSQword,
    /// 双精度浮点数转无符号四字
    OpConvertDoubleToUQword,
    /// 双精度浮点数转有符号八字
    OpConvertDoubleToSOword,
    /// 双精度浮点数转无符号八字
    OpConvertDoubleToUOword,
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
    /// 八字转布尔型
    OpConvertOwordToBool,
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
    /// 八字位取反
    OpBitNotOword,
    /// 字节位与
    OpBitAndByte,
    /// 单字位与
    OpBitAndWord,
    /// 双字位与
    OpBitAndDword,
    /// 四字位与
    OpBitAndQword,
    /// 八字位与
    OpBitAndOword,
    /// 字节位或
    OpBitOrByte,
    /// 单字位或
    OpBitOrWord,
    /// 双字位或
    OpBitOrDword,
    /// 四字位或
    OpBitOrQword,
    /// 八字位或
    OpBitOrOword,
    /// 字节位异或
    OpBitXorByte,
    /// 单字位异或
    OpBitXorWord,
    /// 双字位异或
    OpBitXorDword,
    /// 四字位异或
    OpBitXorQword,
    /// 八字位异或
    OpBitXorOword,
    /// 字节左位移
    OpShiftLeftByte,
    /// 单字左位移
    OpShiftLeftWord,
    /// 双字左位移
    OpShiftLeftDword,
    /// 四字左位移
    OpShiftLeftQword,
    /// 八字左位移
    OpShiftLeftOword,
    /// 字节符号右位移
    OpSignShiftRightByte,
    /// 单字符号右位移
    OpSignShiftRightWord,
    /// 双字符号右位移
    OpSignShiftRightDword,
    /// 四字符号右位移
    OpSignShiftRightQword,
    /// 八字符号右位移
    OpSignShiftRightOword,
    /// 字节零右位移
    OpZeroShiftRightByte,
    /// 单字零右位移
    OpZeroShiftRightWord,
    /// 双字零右位移
    OpZeroShiftRightDword,
    /// 四字零右位移
    OpZeroShiftRightQword,
    /// 八字零右位移
    OpZeroShiftRightOword,
    /// 字节比较等于
    OpICmpEqualByte,
    /// 单字比较等于
    OpICmpEqualWord,
    /// 双字比较等于
    OpICmpEqualDword,
    /// 四字比较等于
    OpICmpEqualQword,
    /// 八字比较等于
    OpICmpEqualOword,
    /// 字节比较不等于
    OpICmpNotEqualByte,
    /// 单字比较不等于
    OpICmpNotEqualWord,
    /// 双字比较不等于
    OpICmpNotEqualDword,
    /// 四字比较不等于
    OpICmpNotEqualQword,
    /// 八字比较不等于
    OpICmpNotEqualOword,
    /// 有符号字节比较小于
    OpICmpLessSByte,
    /// 有符号单字比较小于
    OpICmpLessSWord,
    /// 有符号双字比较小于
    OpICmpLessSDword,
    /// 有符号四字比较小于
    OpICmpLessSQword,
    /// 有符号八字比较小于
    OpICmpLessSOword,
    /// 无符号字节比较小于
    OpICmpLessUByte,
    /// 无符号单字比较小于
    OpICmpLessUWord,
    /// 无符号双字比较小于
    OpICmpLessUDword,
    /// 无符号四字比较小于
    OpICmpLessUQword,
    /// 无符号八字比较小于
    OpICmpLessUOword,
    /// 有符号字节比较小于等于
    OpICmpLessEqualSByte,
    /// 有符号单字比较小于等于
    OpICmpLessEqualSWord,
    /// 有符号双字比较小于等于
    OpICmpLessEqualSDword,
    /// 有符号四字比较小于等于
    OpICmpLessEqualSQword,
    /// 有符号八字比较小于等于
    OpICmpLessEqualSOword,
    /// 无符号字节比较小于等于
    OpICmpLessEqualUByte,
    /// 无符号单字比较小于等于
    OpICmpLessEqualUWord,
    /// 无符号双字比较小于等于
    OpICmpLessEqualUDword,
    /// 无符号四字比较小于等于
    OpICmpLessEqualUQword,
    /// 无符号八字比较小于等于
    OpICmpLessEqualUOword,
    /// 有符号字节比较大于
    OpICmpGreaterSByte,
    /// 有符号单字比较大于
    OpICmpGreaterSWord,
    /// 有符号双字比较大于
    OpICmpGreaterSDword,
    /// 有符号四字比较大于
    OpICmpGreaterSQword,
    /// 有符号八字比较大于
    OpICmpGreaterSOword,
    /// 无符号字节比较大于
    OpICmpGreaterUByte,
    /// 无符号单字比较大于
    OpICmpGreaterUWord,
    /// 无符号双字比较大于
    OpICmpGreaterUDword,
    /// 无符号四字比较大于
    OpICmpGreaterUQword,
    /// 无符号八字比较大于
    OpICmpGreaterUOword,
    /// 有符号字节比较大于等于
    OpICmpGreaterEqualSByte,
    /// 有符号单字比较大于等于
    OpICmpGreaterEqualSWord,
    /// 有符号双字比较大于等于
    OpICmpGreaterEqualSDword,
    /// 有符号四字比较大于等于
    OpICmpGreaterEqualSQword,
    /// 有符号八字比较大于等于
    OpICmpGreaterEqualSOword,
    /// 无符号字节比较大于等于
    OpICmpGreaterEqualUByte,
    /// 无符号单字比较大于等于
    OpICmpGreaterEqualUWord,
    /// 无符号双字比较大于等于
    OpICmpGreaterEqualUDword,
    /// 无符号四字比较大于等于
    OpICmpGreaterEqualUQword,
    /// 无符号八字比较大于等于
    OpICmpGreaterEqualUOword,
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
    /// 弹出八字
    OpPopOword,
    /// 压入字节
    OpPushByte,
    /// 压入单字
    OpPushWord,
    /// 压入双字
    OpPushDword,
    /// 压入四字
    OpPushQword,
    /// 压入八字
    OpPushOword,
    /// 复制字节
    OpCopyByte,
    /// 复制单字
    OpCopyWord,
    /// 复制双字
    OpCopyDword,
    /// 复制四字
    OpCopyQword,
    /// 复制八字
    OpCopyOword,
    /// 获取局部变量字节
    OpGetLocalByte,
    /// 获取局部变量单字
    OpGetLocalWord,
    /// 获取局部变量双字
    OpGetLocalDword,
    /// 获取局部变量四字
    OpGetLocalQword,
    /// 获取局部变量八字
    OpGetLocalOword,
    /// 设置局部变量字节
    OpSetLocalByte,
    /// 设置局部变量单字
    OpSetLocalWord,
    /// 设置局部变量双字
    OpSetLocalDword,
    /// 设置局部变量四字
    OpSetLocalQword,
    /// 设置局部变量八字
    OpSetLocalOword,
    /// 获取引用字节
    OpGetReferenceByte,
    /// 获取引用单字
    OpGetReferenceWord,
    /// 获取引用双字
    OpGetReferenceDword,
    /// 获取引用四字
    OpGetReferenceQword,
    /// 获取引用八字
    OpGetReferenceOword,
    /// 设置引用字节
    OpSetReferenceByte,
    /// 设置引用单字
    OpSetReferenceWord,
    /// 设置引用双字
    OpSetReferenceDword,
    /// 设置引用四字
    OpSetReferenceQword,
    /// 设置引用八字
    OpSetReferenceOword,
}

#[derive(IntoPrimitive, TryFromPrimitive, Clone)]
#[repr(u8)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum SpecialFunction {
    /// 打印无符号字节
    PrintByte,
    /// 打印有符号字节
    PrintSByte,
    /// 打印有符号短整型
    PrintShort,
    /// 打印无符号短整型
    PrintUShort,
    /// 打印有符号整型
    PrintInt,
    /// 打印无符号整型
    PrintUInt,
    /// 打印有符号长整型
    PrintLong,
    /// 打印无符号长整型
    PrintULong,
    /// 打印有符号扩展整数
    PrintExtInt,
    /// 打印无符号扩展整数
    PrintUExtInt,
    /// 打印单精度浮点型
    PrintFloat,
    /// 打印双精度浮点型
    PrintDouble,
    /// 打印布尔型
    PrintBool,
    /// 打印字符
    PrintChar,
    /// 打印换行符
    PrintNewLine,
}
