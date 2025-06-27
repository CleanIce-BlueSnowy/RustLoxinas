//! 全局编译器模块

mod assistance;
mod builtin_functions;

use crate::errors::error_types::{CompileError, CompileResultList};
use crate::front_compiler::FrontCompiler;
use crate::function::LoxinasFunction;
use crate::stmt::{Stmt, StmtFunc};
use crate::types::ValueType;
use crate::{hashmap, stmt_get_pos};
use std::collections::HashMap;

/// 全局编译器
pub struct GlobalCompiler {
    /// 源代码
    source_code: Vec<Stmt>,
    /// 符号链接表
    link_symbol_list: Vec<LinkSymbol>,
    /// 函数引用表
    func_ref_list: Vec<FunctionReference>,
    /// 函数名称到符号的映射
    func_to_symbol: HashMap<String, Vec<String>>,
    /// 符号到函数引用表的映射
    symbol_to_idx: HashMap<String, usize>,
    /// 函数表（函数语句到函数对象的映射）
    functions: HashMap<*const StmtFunc, LoxinasFunction>,
    /// 代码
    codes: Vec<u8>,
    /// 全局类型
    global_types: HashMap<String, ValueType>,
}

impl GlobalCompiler {
    pub fn new(source: Vec<Stmt>) -> Self {
        Self {
            source_code: source,
            link_symbol_list: vec![LinkSymbol::new(0, "".to_string())],  // 0 号位置作为主函数的占位符
            func_ref_list: vec![FunctionReference::SymbolReference { list_index: 0 }],  // 0 号位置引用主函数
            func_to_symbol: hashmap!(),
            symbol_to_idx: hashmap!(),
            functions: hashmap!(),
            codes: vec![],
            global_types: Self::init_types(),
        }
    }

    pub fn compile(&mut self) -> CompileResultList<()> {
        let mut func_search_list: HashMap<String, Vec<LoxinasFunction>> = Self::init_builtin_functions();
        let mut errors = vec![];
        
        if let Err(mut errs) = self.predefine() {
            errors.append(&mut errs);
        }
        
        // 寻找主函数（入口点在 0x00000000）并检查全局语句合法性
        let mut main_func = None;
        let mut functions = vec![];
        for stmt in &self.source_code {
            match stmt {
                Stmt::Func(stmt_func) => {
                    if let Some(func) = self.functions.get(&(stmt_func.as_ref() as *const StmtFunc)) {
                        if func.get_symbol() == "main$unit" {
                            main_func = Some(func);
                        } else {
                            functions.push(func);
                        }
                        if let Some(ele) = func_search_list.get_mut(&stmt_func.name) {
                            ele.push(func.clone());
                        } else {
                            func_search_list.insert(stmt_func.name.clone(), vec![func.clone()]);
                        }
                    }
                }
                _ => errors.push(CompileError::new(&stmt_get_pos!(stmt), "Invalid global statement.".to_string())),
            }
        }
        
        // 编译主函数
        if let Some(func) = main_func {
            let statements_ptr = if let LoxinasFunction::Normal { chunk, .. } = func { chunk } else { unreachable!(); };
            let statements = unsafe {  // SAFETY: 指向的语句块仍存储在 self.source_code 中
                &**statements_ptr
            };
            let compiler = FrontCompiler::new(statements, func, &func_search_list);
            match compiler.compile() {
                Err(mut errs) => errors.append(&mut errs),
                Ok(mut codes) => {
                    // 获取链接
                    let link = if let FunctionReference::SymbolReference { list_index, .. } = self.func_ref_list[0] {
                        &mut self.link_symbol_list[list_index]
                    } else {
                        unreachable!();
                    };
                    // 更新链接位置
                    link.location = 0;
                    // 替换为直接引用
                    self.func_ref_list[0] = FunctionReference::DirectReference { target: 0 };
                    // 写入代码
                    self.codes.append(&mut codes);
                }
            }
        }
        
        // 编译其他函数
        for func in functions {
            let statements_ptr = if let LoxinasFunction::Normal { chunk, .. } = func { chunk } else { unreachable!(); };
            let statements = unsafe {  // SAFETY: 指向的语句块仍存储在 self.source_code 中
                &**statements_ptr
            };
            let compiler = FrontCompiler::new(statements, func, &func_search_list);
            match compiler.compile() {
                Err(mut errs) => errors.append(&mut errs),
                Ok(mut codes) => {
                    let location = self.codes.len();  // 函数起点位置
                    let symbol = func.get_symbol();
                    // 获取链接
                    let link = if let FunctionReference::SymbolReference { list_index, .. } = self.func_ref_list[self.symbol_to_idx[symbol]] {
                        &mut self.link_symbol_list[list_index]
                    } else {
                        unreachable!();
                    };
                    // 更新链接位置
                    link.location = location as isize;
                    // 替换为直接引用
                    self.func_ref_list[self.symbol_to_idx[symbol]] = FunctionReference::DirectReference { target: location };
                    assert!(matches!(self.func_ref_list[self.symbol_to_idx[symbol]], FunctionReference::DirectReference { .. }));
                    // 写入代码
                    self.codes.append(&mut codes);
                }
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    /** 转换为字节码文件
    
    格式为：
    
    - 最前面是三组四个字节的整数（小端法）：第一组表示符号表起始位置，第二组表示函数引用表起始位置，第三组表示代码起始位置。
    - 接着是符号表。
    - 接着是函数引用表。
    - 接着是字节代码。
        */
    pub fn get_bytes(mut self) -> Vec<u8> {
        let mut res = vec![];
        res.extend(12_u32.to_ne_bytes());  // 符号表起始位置已确定
        res.extend(0_u64.to_ne_bytes());  // 占位符
        res.append(&mut self.symbol_list_to_binary());
        let length = res.len() as u32;
        res[4..8].copy_from_slice(&length.to_le_bytes());
        res.append(&mut self.func_ref_list_to_binary());
        let length = res.len() as u32;
        res[8..12].copy_from_slice(&length.to_le_bytes());
        res.append(&mut self.codes);
        res
    }
}

/// 链接符号
struct LinkSymbol {
    /// 符号文件位置。0 表示当前文件
    position: u32,
    /// 符号
    symbol: String,
    /// 函数在本文件代码中的位置，-1 表示默认无效
    location: isize,
}

impl LinkSymbol {
    fn new(position: u32, symbol: String) -> Self {
        Self {
            position,
            symbol,
            location: -1,
        }
    }
}

/// 函数引用
enum FunctionReference {
    /// 直接函数引用
    DirectReference {
        /// 函数在代码中的位置
        target: usize,
    },
    /// 符号函数引用
    SymbolReference {
        /// 引用表的位置
        list_index: usize,
    }
}
