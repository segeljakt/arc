//! Macros for using arbitrary enums without having to
//! implement a bunch of boilerplate methods.
//!
//! Requirements which must be satisfied by arc-lang:
//! * Each enum variant has exactly one field.
//! * Each enum variant identifier is globally unique.
//!
//! All of these assumptions will be ensured by the Codegen interface.

/// Enwraps a value into an enum-variant.
///
/// ```
/// mod foo {
///     #[arc_runtime::prelude::rewrite]
///     pub enum Bar {
///         Baz(i32),
///         Qux(i32)
///     }
/// }
/// let x = arc_runtime::prelude::enwrap!(foo::Bar::Baz, 5);
/// ```
#[macro_export]
macro_rules! enwrap {
    (@done $path:path, $expr:expr) => {
        $path($expr).into()
    };
    ($mod:ident :: $enum:ident :: $variant:ident , $expr:expr) => {
        arc_runtime::prelude::paste!(arc_runtime::prelude::enwrap!(@done $mod::[<Concrete $enum>]::$variant, $expr))
    };
    ($enum:ident :: $variant:ident , $expr:expr) => {
        arc_runtime::prelude::paste!(arc_runtime::prelude::enwrap!(@done [<Concrete $enum>]::$variant, $expr))
    };
    ($variant:ident , $expr:expr) => {
        arc_runtime::prelude::enwrap!(@done $variant, $expr)
    };
}

/// Returns `true` if enum is a certain variant, else `false`.
///
/// ```
/// mod foo {
///     #[arc_runtime::prelude::rewrite]
///     pub enum Bar {
///         Baz(i32),
///         Qux(i32)
///     }
/// }
///
/// let x = arc_runtime::prelude::enwrap!(foo::Bar::Baz, 5);
/// assert!(arc_runtime::prelude::is!(foo::Bar::Baz, x));
/// ```
#[macro_export]
macro_rules! is {
    (@done $path:path, $expr:expr) => {
        if let $path(_) = $expr.0.as_ref() {
            true
        } else {
            false
        }
    };
    ($mod:ident :: $enum:ident :: $variant:ident , $expr:expr) => {
        arc_runtime::prelude::paste!(arc_runtime::prelude::is!(@done $mod::[<Concrete $enum>]::$variant, $expr))
    };
    ($enum:ident :: $variant:ident , $expr:expr) => {
        arc_runtime::prelude::paste!(arc_runtime::prelude::is!(@done [<Concrete $enum>]::$variant, $expr))
    };
    ($variant:ident , $expr:expr) => {
        arc_runtime::prelude::is!(@done $variant, $expr)
    };
}

/// Unwraps a value out of an enum-variant.
///
/// ```
/// mod foo {
///     #[arc_runtime::prelude::rewrite]
///     pub enum Bar {
///         Baz(i32),
///         Qux(i32)
///     }
/// }
///
/// let x = arc_runtime::prelude::enwrap!(foo::Bar::Baz, 5);
/// let y = arc_runtime::prelude::unwrap!(foo::Bar::Baz, x);
/// ```
#[macro_export]
macro_rules! unwrap {
    (@done $path:path, $expr:expr) => {
        if let $path(v) = $expr.0.as_ref() {
            v.clone()
        } else {
            unreachable!()
        }
    };
    ($mod:ident :: $enum:ident :: $variant:ident , $expr:expr) => {
        arc_runtime::prelude::paste!(arc_runtime::prelude::unwrap!(@done $mod::[<Concrete $enum>]::$variant, $expr))
    };
    ($enum:ident :: $variant:ident , $expr:expr) => {
        arc_runtime::prelude::paste!(arc_runtime::prelude::unwrap!(@done [<Concrete $enum>]::$variant, $expr))
    };
    ($variant:ident , $expr:expr) => {
        arc_runtime::prelude::unwrap!(@done $variant, $expr)
    };
}
