//! 语义分析模块

use crate::global_compiler::GlobalCompiler;
use crate::hashmap;
use crate::tokens::Token;
use crate::types::ValueType;
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;

mod resolver_assistance;
mod resolver_expr;
mod resolver_stmt;

/// 语义分析器
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Resolver {
    pub global_types: HashMap<String, ValueType>,
    pub scopes: Vec<Scope>,
    pub now_slot: usize,
}

impl Resolver {
    #[must_use]
    pub fn new() -> Self {
        Self {
            global_types: GlobalCompiler::init_types(),
            scopes: vec![],
            now_slot: 0,
        }
    }
    
    /// 初始化形参
    pub fn init_parameters(&mut self, params: &[(String, ValueType)]) {
        let scope = &mut self.scopes[0];
        for param in params {
            scope.add_variable(param.0.clone(), Variable::new(true, true, self.now_slot, Some(param.1.clone()), false));
            self.now_slot += param.1.get_size().get_bytes_cnt();
        }
    }

    /// 操作符转字符串，方便报错
    #[must_use]
    pub fn operator_to_string(token: &Rc<Token>) -> String {
        use crate::tokens::TokenType;
        String::from(match &token.token_type {
            TokenType::Operator(operator) => {
                use crate::tokens::TokenOperator::*;

                match operator {
                    Plus => "+",
                    Minus => "-",
                    Star => "*",
                    Slash => "/",
                    Power => "**",
                    Comma => ",",
                    Backslash => "\\",
                    And => "&",
                    Pipe => "|",
                    Tilde => "~",
                    Colon => ":",
                    Semicolon => ";",
                    Equal => "=",
                    EqualEqual => "==",
                    NotEqual => "!=",
                    Less => "<",
                    LessEqual => "<=",
                    Greater => ">",
                    GreaterEqual => ">=",
                    Bang => "!",
                    Caret => "^",
                    Mod => "%",
                    DoubleColon => "::",
                    PlusEqual => "+=",
                    MinusEqual => "-=",
                    StarEqual => "*=",
                    SlashEqual => "/=",
                    AndEqual => "&=",
                    PipeEqual => "|=",
                    CaretEqual => "^=",
                    ModEqual => "%=",
                    RightArrow => "->",
                }
            }
            TokenType::Keyword(operator) => {
                use crate::tokens::TokenKeyword::*;
                match operator {
                    And => "and",
                    Or => "or",
                    Not => "not",
                    Shl => "shl",
                    Shr => "shr",
                    _ => unreachable!("Invalid operator"),
                }
            }
            _ => unreachable!("Invalid operator"),
        })
    }
}

/// 表达式语义分析结果
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct ExprResolveRes {
    /// 表达式得到的结果类型
    pub res_type: ValueType,
    /// 操作时需要的数据类型
    pub ope_type: ValueType,
}

impl ExprResolveRes {
    #[must_use]
    pub fn new(res_type: ValueType, ope_type: ValueType) -> Self {
        Self {
            res_type,
            ope_type,
        }
    }
}

/// 变量
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct Variable {
    /// 已定义
    pub defined: bool,
    /// 已初始化
    pub initialized: bool,
    /// 栈偏移量
    pub slot: usize,
    /// 变量类型
    pub var_type: Option<ValueType>,
    /// 是否为引用
    pub is_ref: bool,
}

impl Variable {
    #[must_use]
    pub fn new(
        defined: bool,
        initialized: bool,
        slot: usize,
        var_type: Option<ValueType>,
        is_ref: bool,
    ) -> Self {
        Self {
            defined,
            initialized,
            slot,
            var_type,
            is_ref,
        }
    }
}

/// 作用域
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Scope {
    /// 作用域下的变量
    pub variables: HashMap<String, Variable>,
    /// 顶部偏移量
    pub top_slot: usize,
    /// 作用域内初始化的变量，有序方便比较
    pub init_vars: BTreeSet<*mut Variable>,
}

impl Scope {
    #[must_use]
    fn new(now_slot: usize) -> Self {
        Self {
            variables: hashmap!(),
            top_slot: now_slot,
            init_vars: BTreeSet::new(),
        }
    }
    
    // 添加变量
    fn add_variable(&mut self, name: String, var: Variable) {
        self.variables.insert(name, var);
    }
}
