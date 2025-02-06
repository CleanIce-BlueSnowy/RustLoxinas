//! 对象模块

use std::collections::HashMap;
use std::rc::Rc;
use lazy_static::lazy_static;
use crate::hashmap;
use crate::types::ValueType;

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum LoxinasClass {
    Object,
    String,
    UserDefined(Rc<UserClass>),
}

lazy_static!{
    static ref OBJECT_CONTAIN_TYPES: HashMap<String, ValueType> = hashmap!{};
    static ref STRING_CONTAIN_TYPES: HashMap<String, ValueType> = {
        let mut map = hashmap!{};
        map.extend(OBJECT_CONTAIN_TYPES.clone());
        map
    };
}

impl LoxinasClass {
    pub fn get_contain_types(&self) -> &HashMap<String, ValueType> {
        match self {
            LoxinasClass::Object => &*OBJECT_CONTAIN_TYPES,
            LoxinasClass::String => &*STRING_CONTAIN_TYPES,
            LoxinasClass::UserDefined(class) => &class.contain_types,
        }
    }
    
    pub fn get_name(&self) -> &str {
        match self {
            LoxinasClass::Object => "Object",
            LoxinasClass::String => "String",
            LoxinasClass::UserDefined(class) => &class.name,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserClass {
    pub name: String,
    pub contain_types: HashMap<String, ValueType>,
}

impl UserClass {
    pub fn new(name: String) -> Self {
        Self { name, contain_types: hashmap!{} }
    }
}
