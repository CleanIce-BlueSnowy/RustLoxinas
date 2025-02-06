#[macro_export]
macro_rules! hashmap {
    () => {
        HashMap::new()
    };
    ( $( $key:expr => $value:expr ),+ $(,)? ) => {
        HashMap::from([
            $(
                ($key, $value),
            )*
        ])
    }
}
