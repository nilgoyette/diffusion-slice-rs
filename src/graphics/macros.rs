macro_rules! label {
    ($($arg:tt)*) => {
        Some(&format!($($arg)*))
    };
}
