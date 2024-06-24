#[macro_export]
macro_rules! bitwise_contains {
    ($data: expr, $flag: expr) => {
        $data & $flag == $flag
    };
}