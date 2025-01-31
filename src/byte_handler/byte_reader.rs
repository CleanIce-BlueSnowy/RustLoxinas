//! 字节流读取模块

/// 检查安全性——是否会越界
#[inline]
pub fn check_safety(stream: &[u8], offset: usize, need: usize) -> bool {
    offset + need - 1 < stream.len()
}

/// 读取字节
#[inline]
pub fn read_byte(stream: &[u8], offset: usize) -> Result<([u8; 1], usize), ()> {
    if check_safety(stream, offset, 1) {
        Ok(([stream[offset]], offset + 1))
    } else {
        Err(())
    }
}

/// 读取单字
#[inline]
pub fn read_word(stream: &[u8], offset: usize) -> Result<([u8; 2], usize), ()> {
    if check_safety(stream, offset, 2) {
        Ok(([stream[offset], stream[offset + 1]], offset + 2))
    } else {
        Err(())
    }
}

/// 读取双字
#[inline]
pub fn read_dword(stream: &[u8], offset: usize) -> Result<([u8; 4], usize), ()> {
    if check_safety(stream, offset, 4) {
        Ok(([stream[offset], stream[offset + 1], stream[offset + 2], stream[offset + 3]], offset + 4))
    } else {
        Err(())
    }
}

/// 读取四字
#[inline]
pub fn read_qword(stream: &[u8], offset: usize) -> Result<([u8; 8], usize), ()> {
    if check_safety(stream, offset, 8) {
        Ok(([stream[offset], stream[offset + 1], stream[offset + 2], stream[offset + 3],
                stream[offset + 4], stream[offset + 5], stream[offset + 6], stream[offset + 7]], offset + 8))
    } else {
        Err(())
    }
}

/// 读取扩展整数（八字）
#[inline]
pub fn read_extend(stream: &[u8], offset: usize) -> Result<([u8; 16], usize), ()> {
    if check_safety(stream, offset, 16) {
        Ok(([stream[offset], stream[offset + 1], stream[offset + 2], stream[offset + 3],
                stream[offset + 4], stream[offset + 5], stream[offset + 6], stream[offset + 7],
                stream[offset + 8], stream[offset + 9], stream[offset + 10], stream[offset + 11],
                stream[offset + 12], stream[offset + 13], stream[offset + 14], stream[offset + 15]], offset + 16))
    } else {
        Err(())
    }
}
