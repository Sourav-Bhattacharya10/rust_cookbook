#[macro_export]
macro_rules! list {
    ($($x:expr),*) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }};
    ($elem:expr; $n:expr) => {{
        let mut temp_vec = Vec::new();
        for _ in 0..$n {
            temp_vec.push($elem);
        }
        temp_vec
    }};
}
