#[macro_export]
macro_rules! bitwise_contains {
    ($data: expr, $flag: expr) => {
        $data & $flag == $flag
    };
}

#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + $crate::count!($($xs)*));
}

#[macro_export]
/// https://stackoverflow.com/a/64678145/10854888
macro_rules! iterable_enum {
    ($(#[$derives:meta])* $visibility:vis enum $name:ident { $($(#[$nested_meta:meta])* $member:ident = $value: expr),* }) => {
        const COUNT_MEMBERS:usize = $crate::count!($($member)*);
        $(#[$derives])*
        $visibility enum $name {
            $($member = $value),*
        }
        impl $name {
            pub const fn iter() -> [$name; COUNT_MEMBERS] {
                [$($name::$member),*]
            }
        }
    };
}
