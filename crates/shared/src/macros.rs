#[macro_export]
macro_rules! err {
    ($fmt:expr) => { Err($crate::error::Error::Info(format!($fmt))) };
    ($fmt:expr, $($args:tt)*) =>{ Err($crate::error::Error::Info(format!($fmt,$($args)*))) }
}
