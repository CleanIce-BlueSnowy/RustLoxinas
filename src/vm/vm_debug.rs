//! 虚拟机——调试模块

#[cfg(debug_assertions)]
use crate::vm::VM;

#[cfg(debug_assertions)]
impl<'a> VM<'a> {
    /// 打印虚拟机栈
    pub fn print_stack(&self) {
        print!("\nSTACK: {{ ");
        for byte in &self.vm_stack {
            print!("{:02X} ", byte);
        }
        println!("}}");
    }
}
