 #[macro_export]
macro_rules! convert_enum{($src: ident, $dst: ident, $($variant: ident,)*)=> {
    impl Into<$dst> for $src {
        fn into(self) -> $dst {
            match self {
                $(Self::$variant => $dst::$variant,)*
            }
        }
    }
}}