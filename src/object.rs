/// Loxinas 语言中的类
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum LoxinasClass {
    /// Loxinas 内置类型 `String`
    LoxinasString,
    /// 用户自定义类型
    UserClass,
}
