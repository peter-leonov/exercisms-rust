#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + $crate::count!($($xs)*));
}

#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ($($ls:expr => $rs:expr),+ $(,)?) => {{
        let mut m = ::std::collections::HashMap::with_capacity($crate::count!($($ls)*));
        $(m.insert($ls, $rs);)*
        m
    }};
}
