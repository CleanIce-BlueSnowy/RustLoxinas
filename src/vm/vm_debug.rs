//! 虚拟机——调试模块

#[cfg(debug_assertions)]
use crate::vm::VM;

#[cfg(debug_assertions)]
impl<'a> VM<'a> {
    /// 打印虚拟机栈
    pub fn print_vm_stack(&self) {
        print!("\nVM STACK: {{ ");
        for byte in &self.vm_stack {
            print!("{:02X} ", byte);
        }
        println!("}}");
    }
    
    /// 打印指令指针栈
    pub fn print_ip_stack(&self) {
        print!("IP STACK: {{ ");
        for ip in &self.ip_stack {
            print!("{:08X} ", ip);
        }
        println!("}}");
    }
    
    /// 打印栈帧起点栈
    pub fn print_frame_stack(&self) {
        print!("FRAME STACK: {{ ");
        for frame_start in &self.frame_stack {
            print!("{} ", frame_start);
        }
        println!("}}");
    }
}
