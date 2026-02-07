#[macro_export]
macro_rules! hashmap {
    ($($key:tt => $val:expr),* $(,)*) => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key, $val); )*
        map
    }};
}

pub mod compile_fail_tests;
