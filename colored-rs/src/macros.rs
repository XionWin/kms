#[macro_export]
#[doc="print_hight_light"]
macro_rules! print_hight_light {
    () => {
        println!()
    };
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        println!("{}", $crate::Colorize::bright_white(&*msg))
    }};
}

#[macro_export]
#[doc="print_info"]
macro_rules! print_info {
    () => {
        println!()
    };
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        println!("{}", $crate::Colorize::white(&*msg))
    }};
}

#[macro_export]
#[doc="print_debug"]
macro_rules! print_debug {
    () => {
        println!()
    };
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        println!("{}", $crate::Colorize::bright_black(&*msg))
    }};
}

#[macro_export]
#[doc="print_warning"]
macro_rules! print_warning {
    () => {
        println!()
    };
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        println!("{}", $crate::Colorize::yellow(&*msg))
    }};
}

#[macro_export]
#[doc="print_error"]
macro_rules! print_error {
    () => {
        println!()
    };
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        println!("{}", $crate::Colorize::red(&*msg))
    }};
}
