
#[macro_export]
macro_rules! log {
    ($($tt:tt)*) => {
        info!(log, $($tt)*);
    };
}
