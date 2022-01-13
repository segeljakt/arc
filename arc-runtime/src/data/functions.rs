/// Construct a new function value.
#[macro_export]
macro_rules! fun_val {
    { $arg:expr } => { $arg }
}

/// Construct a new function type.
#[macro_export]
macro_rules! fun_type {
    { ($($inputs:ty),+) -> $output:ty } => { fn($($inputs),+) -> $output }
}
