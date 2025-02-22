//! 辅助功能模块

/// 快速创建散列表
#[macro_export]
macro_rules! hashmap {
    () => {
        {
            use std::collections::HashMap;
            HashMap::new()
        }
    };
    ( $( $key:expr => $value:expr ),+ $(,)? ) => {
        {
            use std::collections::HashMap;
            HashMap::from([
                $(
                    ($key, $value),
                )*
            ])
        }
    }
}
