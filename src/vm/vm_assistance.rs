//! 虚拟机——辅助功能模块

use crate::vm::VM;

impl<'a> VM<'a> {
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
    pub fn push_extend(&mut self, extend: [u8; 16]) {
        self.vm_stack.extend_from_slice(&extend);
    }
    
    #[inline]
    pub fn push_bool(&mut self, value: bool) {
        self.vm_stack.push(
            if value {
                0x01
            } else {
                0x00
            }
        );
    }

    #[inline]
    pub fn peek_byte(&self) -> [u8; 1] {
        let top = self.vm_stack.len() - 1;
        return [self.vm_stack[top]];
    }

    #[inline]
    pub fn peek_word(&self) -> [u8; 2] {
        let top = self.vm_stack.len() - 1;
        return [self.vm_stack[top - 1], self.vm_stack[top]];
    }

    #[inline]
    pub fn peek_dword(&self) -> [u8; 4] {
        let top = self.vm_stack.len() - 1;
        return [self.vm_stack[top - 3], self.vm_stack[top - 2], self.vm_stack[top - 1], self.vm_stack[top]];
    }

    #[inline]
    pub fn peek_qword(&self) -> [u8; 8] {
        let top = self.vm_stack.len() - 1;
        return [
            self.vm_stack[top - 7], self.vm_stack[top - 6], self.vm_stack[top - 5], self.vm_stack[top - 4],
            self.vm_stack[top - 3], self.vm_stack[top - 2], self.vm_stack[top - 1], self.vm_stack[top]
        ];
    }

    #[inline]
    pub fn peek_extend(&self) -> [u8; 16] {
        let top = self.vm_stack.len() - 1;
        return [
            self.vm_stack[top - 15], self.vm_stack[top - 14], self.vm_stack[top - 13], self.vm_stack[top - 12],
            self.vm_stack[top - 11], self.vm_stack[top - 10], self.vm_stack[top - 9], self.vm_stack[top - 8],
            self.vm_stack[top - 7], self.vm_stack[top - 6], self.vm_stack[top - 5], self.vm_stack[top - 4],
            self.vm_stack[top - 3], self.vm_stack[top - 2], self.vm_stack[top - 1], self.vm_stack[top]
        ];
    }
    
    #[inline]
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
    pub fn pop_extend(&mut self) -> [u8; 16] {
        let res = self.peek_extend();
        self.vm_stack.truncate(self.vm_stack.len() - 16);
        return res;
    }
    
    #[inline]
    pub fn pop_bool(&mut self) -> bool {
        let res = self.peek_bool();
        self.vm_stack.truncate(self.vm_stack.len() - 1);
        return res;
    }
}
