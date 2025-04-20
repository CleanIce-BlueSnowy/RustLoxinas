//! 虚拟机——辅助功能模块

use crate::byte_handler::byte_reader::{read_byte, read_dword, read_oword, read_qword, read_word};
use crate::vm::VM;

impl<'a> VM<'a> {
    #[inline]
    pub fn jump(&mut self, goto: i32) {
        self.ip = (self.ip as isize + goto as isize) as usize;
    }

    #[inline]
    pub fn stack_shrink(&mut self, length: u32) {
        let old_length = self.vm_stack.len();
        self.vm_stack.truncate(old_length - length as usize);
    }

    #[inline]
    pub fn stack_extend(&mut self, length: u32) {
        let old_length = self.vm_stack.len();
        self.vm_stack.resize(old_length + length as usize, 0x00);
    }

    #[inline]
    pub fn push_byte(&mut self, byte: [u8; 1]) {
        self.vm_stack.extend_from_slice(&byte);
    }

    #[inline]
    pub fn push_word(&mut self, word: [u8; 2]) {
        self.vm_stack.extend_from_slice(&word);
    }

    #[inline]
    pub fn push_dword(&mut self, dword: [u8; 4]) {
        self.vm_stack.extend_from_slice(&dword);
    }

    #[inline]
    pub fn push_qword(&mut self, qword: [u8; 8]) {
        self.vm_stack.extend_from_slice(&qword);
    }

    #[inline]
    pub fn push_oword(&mut self, oword: [u8; 16]) {
        self.vm_stack.extend_from_slice(&oword);
    }

    #[inline]
    pub fn push_bool(&mut self, value: bool) {
        self.vm_stack.push(if value { 0x01 } else { 0x00 });
    }

    #[inline]
    #[must_use]
    pub fn peek_byte(&self) -> [u8; 1] {
        let top = self.vm_stack.len() - 1;
        return [self.vm_stack[top]];
    }

    #[inline]
    #[must_use]
    pub fn peek_word(&self) -> [u8; 2] {
        let top = self.vm_stack.len() - 1;
        return [self.vm_stack[top - 1], self.vm_stack[top]];
    }

    #[inline]
    #[must_use]
    pub fn peek_dword(&self) -> [u8; 4] {
        let top = self.vm_stack.len() - 1;
        return [
            self.vm_stack[top - 3],
            self.vm_stack[top - 2],
            self.vm_stack[top - 1],
            self.vm_stack[top],
        ];
    }

    #[inline]
    #[must_use]
    pub fn peek_qword(&self) -> [u8; 8] {
        let top = self.vm_stack.len() - 1;
        return [
            self.vm_stack[top - 7],
            self.vm_stack[top - 6],
            self.vm_stack[top - 5],
            self.vm_stack[top - 4],
            self.vm_stack[top - 3],
            self.vm_stack[top - 2],
            self.vm_stack[top - 1],
            self.vm_stack[top],
        ];
    }

    #[inline]
    #[must_use]
    pub fn peek_oword(&self) -> [u8; 16] {
        let top = self.vm_stack.len() - 1;
        return [
            self.vm_stack[top - 15],
            self.vm_stack[top - 14],
            self.vm_stack[top - 13],
            self.vm_stack[top - 12],
            self.vm_stack[top - 11],
            self.vm_stack[top - 10],
            self.vm_stack[top - 9],
            self.vm_stack[top - 8],
            self.vm_stack[top - 7],
            self.vm_stack[top - 6],
            self.vm_stack[top - 5],
            self.vm_stack[top - 4],
            self.vm_stack[top - 3],
            self.vm_stack[top - 2],
            self.vm_stack[top - 1],
            self.vm_stack[top],
        ];
    }

    #[inline]
    #[must_use]
    pub fn peek_bool(&self) -> bool {
        let top = self.vm_stack.len() - 1;
        return self.vm_stack[top] != 0;
    }

    #[inline]
    pub fn pop_byte(&mut self) -> [u8; 1] {
        let res = self.peek_byte();
        self.vm_stack.truncate(self.vm_stack.len() - 1);
        return res;
    }

    #[inline]
    pub fn pop_word(&mut self) -> [u8; 2] {
        let res = self.peek_word();
        self.vm_stack.truncate(self.vm_stack.len() - 2);
        return res;
    }

    #[inline]
    pub fn pop_dword(&mut self) -> [u8; 4] {
        let res = self.peek_dword();
        self.vm_stack.truncate(self.vm_stack.len() - 4);
        return res;
    }

    #[inline]
    pub fn pop_qword(&mut self) -> [u8; 8] {
        let res = self.peek_qword();
        self.vm_stack.truncate(self.vm_stack.len() - 8);
        return res;
    }

    #[inline]
    pub fn pop_oword(&mut self) -> [u8; 16] {
        let res = self.peek_oword();
        self.vm_stack.truncate(self.vm_stack.len() - 16);
        return res;
    }

    #[inline]
    pub fn pop_bool(&mut self) -> bool {
        let res = self.peek_bool();
        self.vm_stack.truncate(self.vm_stack.len() - 1);
        return res;
    }

    #[inline]
    #[must_use]
    pub fn read_arg_byte(&mut self) -> [u8; 1] {
        if let Ok((byte, new_ip)) = read_byte(self.chunk, self.ip) {
            self.ip = new_ip;
            byte
        } else {
            panic!("Data not enough: need 1 byte.");
        }
    }

    #[inline]
    #[must_use]
    pub fn read_arg_word(&mut self) -> [u8; 2] {
        if let Ok((word, new_ip)) = read_word(self.chunk, self.ip) {
            self.ip = new_ip;
            word
        } else {
            panic!("Data not enough: need 2 byte.");
        }
    }

    #[inline]
    #[must_use]
    pub fn read_arg_dword(&mut self) -> [u8; 4] {
        if let Ok((dword, new_ip)) = read_dword(self.chunk, self.ip) {
            self.ip = new_ip;
            dword
        } else {
            panic!("Data not enough: need 4 byte.");
        }
    }

    #[inline]
    #[must_use]
    pub fn read_arg_qword(&mut self) -> [u8; 8] {
        if let Ok((qword, new_ip)) = read_qword(self.chunk, self.ip) {
            self.ip = new_ip;
            qword
        } else {
            panic!("Data not enough: need 8 byte.");
        }
    }

    #[inline]
    #[must_use]
    pub fn read_arg_oword(&mut self) -> [u8; 16] {
        if let Ok((oword, new_ip)) = read_oword(self.chunk, self.ip) {
            self.ip = new_ip;
            oword
        } else {
            panic!("Data not enough: need 16 byte.");
        }
    }

    #[inline]
    #[must_use]
    pub fn get_frame_slot_byte(&mut self, slot: usize) -> [u8; 1] {
        if let Ok((byte, _)) = read_byte(&self.vm_stack, self.frame_start + slot) {
            byte
        } else {
            panic!("Data not enough: need 1 byte.");
        }
    }

    #[inline]
    #[must_use]
    pub fn get_frame_slot_word(&mut self, slot: usize) -> [u8; 2] {
        if let Ok((word, _)) = read_word(&self.vm_stack, self.frame_start + slot) {
            word
        } else {
            panic!("Data not enough: need 2 byte.");
        }
    }

    #[inline]
    #[must_use]
    pub fn get_frame_slot_dword(&mut self, slot: usize) -> [u8; 4] {
        if let Ok((dword, _)) = read_dword(&self.vm_stack, self.frame_start + slot) {
            dword
        } else {
            panic!("Data not enough: need 4 byte.");
        }
    }

    #[inline]
    #[must_use]
    pub fn get_frame_slot_qword(&mut self, slot: usize) -> [u8; 8] {
        if let Ok((qword, _)) = read_qword(&self.vm_stack, self.frame_start + slot) {
            qword
        } else {
            panic!("Data not enough: need 8 byte.");
        }
    }

    #[inline]
    #[must_use]
    pub fn get_frame_slot_oword(&mut self, slot: usize) -> [u8; 16] {
        if let Ok((oword, _)) = read_oword(&self.vm_stack, self.frame_start + slot) {
            oword
        } else {
            panic!("Data not enough: need 16 byte.");
        }
    }

    #[inline]
    pub fn set_frame_slot_byte(&mut self, slot: usize, byte: [u8; 1]) {
        self.vm_stack[(self.frame_start + slot)..(self.frame_start + slot + 1)]
            .copy_from_slice(&byte);
    }

    #[inline]
    pub fn set_frame_slot_word(&mut self, slot: usize, word: [u8; 2]) {
        self.vm_stack[(self.frame_start + slot)..(self.frame_start + slot + 2)]
            .copy_from_slice(&word);
    }

    #[inline]
    pub fn set_frame_slot_dword(&mut self, slot: usize, dword: [u8; 4]) {
        self.vm_stack[(self.frame_start + slot)..(self.frame_start + slot + 4)]
            .copy_from_slice(&dword);
    }

    #[inline]
    pub fn set_frame_slot_qword(&mut self, slot: usize, qword: [u8; 8]) {
        self.vm_stack[(self.frame_start + slot)..(self.frame_start + slot + 8)]
            .copy_from_slice(&qword);
    }

    #[inline]
    pub fn set_frame_slot_oword(&mut self, slot: usize, oword: [u8; 16]) {
        self.vm_stack[(self.frame_start + slot)..(self.frame_start + slot + 16)]
            .copy_from_slice(&oword);
    }
}
