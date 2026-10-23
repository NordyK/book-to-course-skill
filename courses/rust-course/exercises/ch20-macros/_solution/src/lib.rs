#[macro_export]
macro_rules! hashmap {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert($k, $v);
        )*
        map
    }};
}

#[macro_export]
macro_rules! min {
    ($x:expr $(,)?) => { $x };
    ($x:expr, $($rest:expr),+ $(,)?) => {{
        let a = $x;
        let b = $crate::min!($($rest),+);
        if a < b { a } else { b }
    }};
}

#[macro_export]
macro_rules! count {
    () => { 0usize };
    ($head:tt $(, $tail:tt)* $(,)?) => { 1usize + $crate::count!($($tail),*) };
}

#[macro_export]
macro_rules! newtype {
    ($name:ident, $inner:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name(pub $inner);

        impl From<$inner> for $name {
            fn from(v: $inner) -> Self {
                $name(v)
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

#[macro_export]
macro_rules! assert_all {
    ($($c:expr),+ $(,)?) => {
        $(
            if !$c {
                panic!("assertion failed: {}", stringify!($c));
            }
        )+
    };
}

#[cfg(test)]
mod tests;
