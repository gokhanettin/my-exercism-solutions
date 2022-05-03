#[macro_export]
macro_rules! hashmap {
    ( $($key:expr => $value:expr),* ) => {
        {
            use ::std::collections::HashMap;
            let mut hm = HashMap::new();
            $(
                hm.insert($key, $value);
            )*
            hm
        }
    };
    ( $($key:expr => $value:expr,)+ ) => {
        {
            use ::std::collections::HashMap;
            let mut hm = HashMap::new();
            $(
                hm.insert($key, $value);
            )*
            hm
        }
    };
}
