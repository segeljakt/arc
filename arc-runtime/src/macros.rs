#[macro_export]
/// Push an event into a channel.
macro_rules! push {
    ($channel:expr, $data:expr) => {
        $channel.push($data).await?
    };
}

#[macro_export]
/// Pull an event from a channel.
macro_rules! pull {
    ($channel:expr) => {
        $channel.pull().await?
    };
}

/// Get the value of a variable.
///
/// ```
/// use arc_runtime::prelude::*;
/// let a = 5;
/// let b = val!(a);
/// ```
#[macro_export]
macro_rules! val {
    ($arg:expr) => {
        $arg.clone()
    };
}

/// Enwraps a value into an enum-variant.
///
/// ```
/// use arc_runtime::prelude::*;
/// mod foo {
///     use arc_runtime::prelude::*;
///     #[rewrite]
///     pub enum Bar {
///         Baz(i32),
///         Qux(i32)
///     }
/// }
/// let x = enwrap!(foo::Bar::Baz, 5);
/// ```
#[macro_export]
macro_rules! enwrap {
    (@done $path:path, $expr:expr, $ctx:expr) => {
        $path($expr).alloc($ctx)
    };
    ($mod:ident :: $enum:ident :: $variant:ident , $expr:expr, $ctx:expr) => {
        paste!(enwrap!(@done $mod::[<Concrete $enum>]::$variant, $expr, $ctx))
    };
    ($enum:ident :: $variant:ident , $expr:expr, $ctx:expr) => {
        paste!(enwrap!(@done [<Concrete $enum>]::$variant, $expr, $ctx))
    };
    ($variant:ident , $expr:expr, $ctx:expr) => {
        enwrap!(@done $variant, $expr, $ctx)
    };
}

/// Returns `true` if enum is a certain variant, else `false`.
///
/// ```
/// use arc_runtime::prelude::*;
/// mod foo {
///     use arc_runtime::prelude::*;
///     #[rewrite]
///     pub enum Bar {
///         Baz(i32),
///         Qux(i32)
///     }
/// }
///
/// let x = enwrap!(foo::Bar::Baz, 5);
/// assert!(is!(foo::Bar::Baz, x));
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
        paste!(is!(@done $mod::[<Concrete $enum>]::$variant, $expr))
    };
    ($enum:ident :: $variant:ident , $expr:expr) => {
        paste!(is!(@done [<Concrete $enum>]::$variant, $expr))
    };
    ($variant:ident , $expr:expr) => {
        is!(@done $variant, $expr)
    };
}

/// Unwraps a value out of an enum-variant.
///
/// ```
/// use arc_runtime::prelude::*;
/// mod foo {
///     use arc_runtime::prelude::*;
///     #[rewrite]
///     pub enum Bar {
///         Baz(i32),
///         Qux(i32)
///     }
/// }
///
/// let x = enwrap!(foo::Bar::Baz, 5);
/// let y = unwrap!(foo::Bar::Baz, x);
/// ```
#[macro_export]
macro_rules! unwrap {
    (@done $path:path, $expr:expr) => {
        if let $path(v) = &*$expr.0 {
            v.clone()
        } else {
            unreachable!()
        }
    };
    ($mod:ident :: $enum:ident :: $variant:ident , $expr:expr) => {
        paste!(unwrap!(@done $mod::[<Concrete $enum>]::$variant, $expr))
    };
    ($enum:ident :: $variant:ident , $expr:expr) => {
        paste!(unwrap!(@done [<Concrete $enum>]::$variant, $expr))
    };
    ($variant:ident , $expr:expr) => {
        unwrap!(@done $variant, $expr)
    };
}

/// Constructs a struct.
///
/// ```
/// use arc_runtime::prelude::*;
/// mod foo {
///     use arc_runtime::prelude::*;
///     #[rewrite]
///     pub struct Bar {
///         x: i32,
///         y: i32
///     }
/// }
/// let x = new!(foo::Bar { x: i32, y: i32 });
/// ```
#[macro_export]
macro_rules! new {
    (@done $path:tt { $($arg:tt)* }, $ctx:expr) => {
        ($path { $($arg)* }).alloc($ctx)
    };
    ($mod:tt :: $struct:tt { $($arg:tt)* }, $ctx:expr) => {
        paste!(new!(@done $mod::[<Concrete $struct>] { $($arg)* }, $ctx))
    };
    ($struct:tt { $($arg:tt)* }, $ctx:expr) => {
        paste!(new!(@done [<Concrete $struct>] { $($arg)* }, $ctx))
    };
}

/// Access a struct's field.
///
/// ```
/// use arc_runtime::prelude::*;
/// mod foo {
///     use arc_runtime::prelude::*;
///     #[rewrite]
///     pub struct Bar {
///         x: i32,
///         y: i32
///     }
/// }
/// let a = new!(Bar { x: i32, y: i32 });
/// let b = access!(a, x);
/// ```
#[macro_export]
macro_rules! access {
    ($arg:expr, $field:tt) => {
        $arg.clone().$field.clone()
    };
}

/// Construct a new function value.
#[macro_export]
macro_rules! fun_val {
    ($arg:expr) => {
        $arg
    };
}

/// Construct a new function type.
#[macro_export]
macro_rules! fun_type {
    (($($inputs:ty),+) -> $output:ty) => {
        fn($($inputs),+) -> $output
    }
}

/// Create a future for pulling data from a channel.
#[macro_export]
macro_rules! pull_transition {
    ($future:pat, $pullable:expr, $state:expr) => {
        let tmp = $pullable.clone();
        let $future = async move { tmp.pull().await }.boxed();
        transition!($state);
    };
}

/// Create a future for pushing data into a channel.
#[macro_export]
macro_rules! push_transition {
    ($future:pat, $pushable:expr, $data:expr, $state:expr) => {
        let tmp = $pushable.clone();
        let $future = async move { tmp.push($data).await }.boxed();
        transition!($state);
    };
}

/// Transition to a new state.
#[macro_export]
macro_rules! transition {
    ($state:expr) => {
        return (Pending, $state.into());
    };
}

/// Terminate the state machine.
#[macro_export]
macro_rules! terminate {
    ($state:expr) => {
        return (Ready(()), $state.into());
    };
}

/// Wait until a future completes.
#[macro_export]
macro_rules! wait {
    ($arg:expr, $cx:expr, $finished:expr, $pending:expr) => {
        match $arg.as_mut().poll($cx) {
            Ready(Finished) => terminate!($finished),
            Ready(Continue(x)) => x,
            Pending => transition!($pending),
        }
    };
}

#[macro_export]
macro_rules! letroot {
    ($var_name:ident : $t:ty  = $stack:expr, $value:expr) => {
        let stack: &ShadowStack = &$stack;
        let value = $value;
        #[allow(unused_unsafe)]
        let mut $var_name = unsafe {
            ShadowStackInternal::<$t>::construct(
                stack,
                stack.head.get(),
                core::mem::transmute::<_, TraitObject>(&value as &dyn Rootable).vtable as usize,
                value,
            )
        };
        #[allow(unused_unsafe)]
        stack.head.set(unsafe { core::mem::transmute(&mut $var_name) });
        #[allow(unused_mut)]
        let mut $var_name = unsafe { Rooted::construct(&mut $var_name.value) };
    };

    ($var_name:ident = $stack:expr, $value:expr) => {
        let stack: &ShadowStack = &$stack;
        let value = $value;
        #[allow(unused_unsafe)]
        let mut $var_name = unsafe {
            ShadowStackInternal::<_>::construct(
                stack,
                stack.head.get(),
                core::mem::transmute::<_, TraitObject>(&value as &dyn Rootable).vtable as usize,
                value,
            )
        };
        #[allow(unused_unsafe)]
        stack.head.set(unsafe { core::mem::transmute(&mut $var_name) });
        #[allow(unused_mut)]
        #[allow(unused_unsafe)]
        let mut $var_name = unsafe { Rooted::construct(&mut $var_name.value) };
    };
}
// comet::gc_vector!()
#[macro_export]
macro_rules! vector {
    ([$($x:expr),+] , $ctx:expr) => {{
        let stack = $ctx.mutator.shadow_stack();
        $crate::letroot!(vec = stack, Some(Vec::new(&mut $ctx)));

        $(
            vec.as_mut().unwrap().0.push(&mut $ctx.mutator, $x);
            vec.as_mut().unwrap().0.write_barrier(&mut $ctx.mutator);
        )*
        vec.take().unwrap()
    }}
}
