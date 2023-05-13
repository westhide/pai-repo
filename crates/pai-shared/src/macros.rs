#[macro_export]
macro_rules! err {
    ($fmt:expr) => { Err($crate::error::PError::Info(format!($fmt))) };
    ($fmt:expr, $($args:tt)*) =>{ Err($crate::error::PError::Info(format!($fmt,$($args)*))) }
}
