//! 对象模块

/// Loxinas 语言中的类
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub enum LoxinasClass {
    /// Loxinas 内置类型 `String`
    LoxinasString,
    /// 用户自定义类型
    UserClass,
}
