//! 虚拟机——调试模块

use crate::vm::VM;

#[cfg(debug_assertions)]
impl<'a> VM<'a> {
    pub fn print_stack(&self) {
        print!("STACK: {{ ");
        for byte in &self.vm_stack {
            print!("{:02X} ", byte);
        }
        println!("}}");
    }
}
