//! Macros for using structs.

/// Constructs a struct.
///
/// ```
/// mod foo {
///     #[arc_runtime::prelude::rewrite]
///     pub struct Bar {
///         x: i32,
///         y: i32
///     }
/// }
/// let x = arc_runtime::prelude::new!(Bar { x: i32, y: i32 });
/// ```
#[macro_export]
macro_rules! new {
    (@done $path:tt { $($arg:tt)* }) => {
        ($path { $($arg)* }).into()
    };
    ($mod:tt :: $struct:tt { $($arg:tt)* }) => {
        arc_runtime::prelude::paste!(arc_runtime::prelude::new!(@done $mod::[<Concrete $struct>] { $($arg)* }))
    };
    ($struct:tt { $($arg:tt)* }) => {
        arc_runtime::prelude::paste!(arc_runtime::prelude::new!(@done [<Concrete $struct>] { $($arg)* }))
    };
}

/// Access a struct's field.
///
/// ```
/// mod foo {
///     #[arc_runtime::prelude::rewrite]
///     pub struct Bar {
///         x: i32,
///         y: i32
///     }
/// }
/// let a = arc_runtime::prelude::new!(Bar { x: i32, y: i32 });
/// let b = arc_runtime::prelude::access!(a, x);
/// ```
#[macro_export]
macro_rules! access {
    ($arg:expr, $field:tt) => {
        $arg.clone().$field.clone()
    };
}
