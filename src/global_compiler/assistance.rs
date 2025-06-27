//! 全局编译器辅助模块

use std::collections::HashMap;
use crate::errors::error_types::{CompileError, CompileResult, CompileResultList};
use crate::function::LoxinasFunction;
use crate::global_compiler::{FunctionReference, GlobalCompiler, LinkSymbol};
use crate::hashmap;
use crate::object::LoxinasClass;
use crate::stmt::{Stmt, StmtFunc};
use crate::types::{TypeTag, ValueType};

impl GlobalCompiler {
    /// 预定义
    pub fn predefine(&mut self) -> CompileResultList<()> {
        let mut err_list = vec![];

        for stmt in &self.source_code {
            match stmt {
                Stmt::Func(stmt_func) => {
                    let ptr = stmt_func.as_ref() as *const StmtFunc;
                    let func_name = stmt_func.name.clone();
                    let mut param_types = vec![];
                    let mut valid = true;
                    for param in &stmt_func.params {
                        match Self::parse_value_type(&self.global_types, &param.1) {
                            Ok(value_type) => param_types.push(value_type),
                            Err(err) => {
                                err_list.push(err);
                                valid = false;
                            }
                        }
                    }
                    let return_type = if let Some(tag) = &stmt_func.return_type {
                        match Self::parse_value_type(&self.global_types, &tag) {
                            Ok(value_type) => value_type,
                            Err(err) => {
                                err_list.push(err);
                                valid = false;
                                ValueType::Unit
                            }
                        }
                    } else {
                        ValueType::Unit
                    };
                    if valid {
                        // 主函数特殊处理
                        if func_name == "main".to_string() {
                            if param_types.len() != 0 {
                                err_list.push(CompileError::new(&stmt_func.pos, "`main` function shouldn't have any parameters.".to_string()));
                                valid = false;
                            }
                            if return_type != ValueType::Unit {
                                err_list.push(CompileError::new(&stmt_func.pos, "`main` function should return `unit`.".to_string()));
                                valid = false;
                            }
                            if self.func_to_symbol.contains_key("main") {
                                err_list.push(CompileError::new(&stmt_func.pos, "`main` functions shouldn't have any overloaded versions.".to_string()));
                                valid = false;
                            }
                            if valid {
                                // 更新符号链接
                                self.link_symbol_list[0].symbol = "main$unit".to_string();
                                // 创建函数名到符号的映射
                                self.func_to_symbol.insert("main".to_string(), vec!["main$unit".to_string()]);
                                // 创建符号到函数引用的映射
                                self.symbol_to_idx.insert("main$unit".to_string(), 0);
                                // 创建函数对象
                                let code_ref = if let Stmt::Block(block) = &stmt_func.body { block } else { unreachable!(); };
                                let function = LoxinasFunction::Normal {
                                    symbol: "main$unit".to_string(),
                                    params: vec![],
                                    return_type: ValueType::Unit,
                                    idx: 0,
                                    chunk: &code_ref.statements,
                                };
                                self.functions.insert(ptr, function);
                            }
                        } else {
                            // 获取符号
                            let symbol = Self::get_function_symbol(&func_name, &param_types, &return_type);
                            // 检查重载合法性
                            if self.func_to_symbol.contains_key(&func_name) {
                                let without_return = symbol.split_at(symbol.find('$').unwrap()).0;
                                for other in self.func_to_symbol.get(&func_name).unwrap() {
                                    let other_without = other.split_at(other.find('$').unwrap()).0;
                                    if without_return == other_without {
                                        err_list.push(CompileError::new(&stmt_func.pos, "Invalid overload: the types of parameters are the same as another.".to_string()));
                                        valid = false;
                                        break;
                                    }
                                }
                                if valid {
                                    // 为函数添加重载符号
                                    self.func_to_symbol.get_mut(&func_name).unwrap().push(symbol.to_string());
                                }
                            } else {
                                // 创建该函数，绑定目前唯一的符号
                                self.func_to_symbol.insert(func_name, vec![symbol.clone()]);
                            }

                            if valid {
                                // 即将创建的符号链接的索引
                                let link_idx = self.link_symbol_list.len();
                                // 创建符号链接
                                self.link_symbol_list.push(LinkSymbol::new(0, symbol.clone()));
                                // 即将创建的函数引用的索引
                                let ref_idx = self.func_ref_list.len();
                                // 创建函数引用
                                self.func_ref_list.push(FunctionReference::SymbolReference { list_index: link_idx });
                                // 创建符号到函数引用的映射
                                self.symbol_to_idx.insert(symbol.clone(), ref_idx);
                                // 创建函数对象
                                let code_ref = if let Stmt::Block(block) = &stmt_func.body { block } else { unreachable!(); };
                                let function = LoxinasFunction::Normal {
                                    symbol,
                                    params: stmt_func.params.iter().map(|ele| ele.0.clone()).zip(param_types.into_iter()).collect(),
                                    return_type,
                                    idx: ref_idx,
                                    chunk: &code_ref.statements,
                                };
                                self.functions.insert(ptr, function);
                            }
                        }
                    }
                }
                _ => ()
            }
        }

        if err_list.is_empty() {
            Ok(())
        } else {
            Err(err_list)
        }
    }
    
    /** 将符号表转换为二进制
    
    格式：
    
    - 最开始的四个字节存放符号表大小。
    - 随后对于每一条符号表条目，先存放四个字节（小端法）表示符号字符串的字节数 n，再存放 n 表示符号表的 UTF-8 二进制编码，随后四个字节（小端法）表示符号定位的文件位置，再随后是四个字节（小端法）表示该函数在该文件中的代码位置（-1 表示未知）。
        */
    pub fn symbol_list_to_binary(&self) -> Vec<u8> {
        let mut res = (self.link_symbol_list.len() as u32).to_le_bytes().to_vec();
        for element in &self.link_symbol_list {
            let bytes_cnt = element.symbol.len() as u32;
            res.extend(bytes_cnt.to_le_bytes());
            let bytes = element.symbol.as_bytes();
            res.extend(bytes);
            let file_pos = element.position;
            res.extend(file_pos.to_le_bytes());
            let location = element.location as i32;
            res.extend(location.to_le_bytes());
        }
        res
    }
    
    /** 将函数引用表转换为二进制
    
    格式：
    
    - 最开始的四个字节存放引用表大小。
    - 随后对于每一个引用表条目，存放四个字节（小端法）表示引用位置，其中若第四个字节（最大端）的第一位为 0，表示符号引用，引用位置为符号表序号；若为 1，表示直接引用，引用位置即函数代码位置。
        */
    pub fn func_ref_list_to_binary(&self) -> Vec<u8> {
        let mut res = (self.func_ref_list.len() as u32).to_le_bytes().to_vec();
        for element in &self.func_ref_list {
            match element {
                FunctionReference::SymbolReference { list_index } => {
                    let mut bytes = (*list_index as u32).to_le_bytes();
                    bytes[3] &= 0b_0111_1111;  // 将第一位设为 0
                    res.extend(bytes);
                }
                FunctionReference::DirectReference { target } => {
                    let mut bytes = (*target as u32).to_le_bytes();
                    bytes[3] |= 0b_1000_0000;  // 将第一为设为 1
                    res.extend(bytes);
                }
            }
        }
        res
    }

    /// 将类型标记转换为数据类型
    pub fn parse_value_type(global_types: &HashMap<String, ValueType>, type_tag: &TypeTag) -> CompileResult<ValueType> {
        use crate::types::ValueType::Object;

        let mut res_type: Option<ValueType> = None;
        let mut search_map = global_types.clone();
        let mut in_global = true;

        for name in &type_tag.chain {
            if let Some(temp_ty) = &res_type {
                if let Object(object) = temp_ty {
                    search_map = object.get_contain_types().clone();
                    in_global = false;
                } else {
                    return Err(CompileError::new(
                        &type_tag.pos,
                        format!("Unknown type '{}' in '{}'.", name, temp_ty),
                    ));
                }
            }
            let ty = if let Some(temp) = search_map.get(name) {
                temp
            } else {
                return Err(CompileError::new(
                    &type_tag.pos,
                    if in_global {
                        format!("Unknown type '{}' in global.", name)
                    } else {
                        format!(
                            "Unknown type '{}' in '{}'.",
                            name,
                            res_type.as_ref().unwrap()
                        )
                    },
                ));
            };
            res_type = Some(ty.clone());
        }

        // 不允许转换为对象
        if let Some(Object(_)) = res_type {
            return Err(CompileError::new(
                &type_tag.pos,
                "Cannot convert a value to an object by using 'as'.".to_string(),
            ));
        }

        Ok(res_type.unwrap())
    }

    /// 初始化全局类型列表
    #[must_use]
    pub fn init_types() -> HashMap<String, ValueType> {
        use crate::types::ValueFloatType::*;
        use crate::types::ValueIntegerType::*;

        hashmap! {
            "char".to_string() => ValueType::Char,
            "bool".to_string() => ValueType::Bool,
            "byte".to_string() => ValueType::Integer(Byte),
            "sbyte".to_string() => ValueType::Integer(SByte),
            "short".to_string() => ValueType::Integer(Short),
            "ushort".to_string() => ValueType::Integer(UShort),
            "int".to_string() => ValueType::Integer(Int),
            "uint".to_string() => ValueType::Integer(UInt),
            "long".to_string() => ValueType::Integer(Long),
            "ulong".to_string() => ValueType::Integer(ULong),
            "extint".to_string() => ValueType::Integer(ExtInt),
            "uextint".to_string() => ValueType::Integer(UExtInt),
            "float".to_string() => ValueType::Float(Float),
            "double".to_string() => ValueType::Float(Double),
            "Object".to_string() => ValueType::Object(LoxinasClass::Object),
            "String".to_string() => ValueType::Object(LoxinasClass::String),
        }
    }

    /** 获取函数符号
   
    格式为：`function_name#param_type1#param_type2#...$return_type`
   
    函数名称在最前面，函数形参类型在中间，每一个前面用 `#` 标记，函数返回类型在最后面，用 `$` 标记

    # 例：
   
    对于这个函数：

    ```loxinas
    func abs(number: int) {
        if number >= 0 {
            return number;
        } else {
            return -number;
        }
    }
    ```

    其符号为 `abs#int$int`

    对于这个函数：

    ```loxinas
    func print_sum(a: double, b: double) {
        print a + b;
    }
    ```

    其符号为 `less_than#double#double$unit`

    对于这个函数：

    ```loxinas
    func get_pi() -> double {
        return 3.1415926;
    }
    ```

    其符号为：`get_pi$double`
      */
    #[must_use]
    pub fn get_function_symbol(name: &str, param_types: &[ValueType], return_type: &ValueType) -> String {
        let mut res = name.to_string();
        for param_type in param_types {
            res.push_str(&format!("#{}", param_type));
        }
        res.push_str(&format!("${}", return_type));
        res
    }
}
