#[macro_export]
macro_rules! hashmap {
    ($($t:tt)*) => {
        todo!()
    };
}

#[macro_export]
macro_rules! min {
    ($($t:tt)*) => {
        todo!()
    };
}

#[macro_export]
macro_rules! count {
    ($($t:tt)*) => {
        todo!()
    };
}

#[macro_export]
macro_rules! newtype {
    ($name:ident, $inner:ty) => {
        // TODO: struct, From, Display
        pub struct $name(pub $inner);
    };
}

#[macro_export]
macro_rules! assert_all {
    ($($t:tt)*) => {};
}

#[cfg(test)]
mod tests;
