//! 字节流写入模块

use std::collections::LinkedList;

/// 写入字节
#[inline]
pub fn write_byte(stream: &mut LinkedList<u8>, byte: [u8; 1]) {
    stream.extend(&byte);
}

/// 写入单字
#[inline]
pub fn write_word(stream: &mut LinkedList<u8>, word: [u8; 2]) {
    stream.extend(&word);
}

/// 写入双字
#[inline]
pub fn write_dword(stream: &mut LinkedList<u8>, dword: [u8; 4]) {
    stream.extend(&dword);
}

/// 写入四字
#[inline]
pub fn write_qword(stream: &mut LinkedList<u8>, qword: [u8; 8]) {
    stream.extend(&qword);
}

/// 写入扩展整数
#[inline]
pub fn write_oword(stream: &mut LinkedList<u8>, oword: [u8; 16]) {
    stream.extend(&oword);
}
