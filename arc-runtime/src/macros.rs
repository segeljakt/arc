/// Push an event into a channel.
#[macro_export]
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
#[cfg(feature = "legacy")]
#[macro_export]
macro_rules! val {
    ($arg:expr) => { $arg.clone() };
}

#[cfg(not(feature = "legacy"))]
#[macro_export]
macro_rules! val {
    ($arg:expr) => { $arg };
}

macro_rules! inline {
    ($($tt:tt)*) => { $($tt)* };
}

/// Access a struct's field.
///
/// ```
/// use arc_runtime::prelude::*;
/// #[rewrite]
/// pub struct Bar {
///     pub x: i32,
///     pub y: i32
/// }
/// #[rewrite(main)]
/// fn main() {
///     let a = new!(Bar { x: 0, y: 1 });
///     let b = access!(a, x);
/// }
/// ```
#[macro_export]
macro_rules! access {
    ($arg:expr, $field:tt) => {
        $arg.clone().$field.clone()
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
        stack
            .head
            .set(unsafe { core::mem::transmute(&mut $var_name) });
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
        stack
            .head
            .set(unsafe { core::mem::transmute(&mut $var_name) });
        #[allow(unused_mut)]
        #[allow(unused_unsafe)]
        let mut $var_name = unsafe { Rooted::construct(&mut $var_name.value) };
    };
}

#[macro_export]
macro_rules! _vector {
    ([$($x:expr),+], $ctx:expr) => {{
        let stack = $ctx.mutator.shadow_stack();
        letroot!(vec = stack, Some(Vec::new($ctx)));

        $(
            vec.as_mut().unwrap().0.push(&mut $ctx.mutator, $x);
            vec.as_mut().unwrap().0.write_barrier(&mut $ctx.mutator);
        )*
        vec.take().unwrap()
    }}
}
