use crate::context::Context;
use crate::data::Data;
use comet::api::Collectable;
use comet::api::Finalize;
use comet::api::Trace;
use erasable::ErasablePtr;
use erasable::ErasedPtr;
use std::fmt::Debug;
use std::ptr::NonNull;

#[macro_export]
macro_rules! declare_functions {
    ($($id:ident),*) => {
        #[derive(Clone)]
        pub struct Function<I: Data, O: Data> {
            pub ptr: fn(I, ErasedPtr, &mut Context) -> O,
            pub tag: FunctionTag,
            pub env: ErasedPtr,
        }
        #[derive(Debug, Clone)]
        pub enum FunctionTag {
            $($id,)*
        }
        impl<I: Data, O: Data> Collectable for Function<I, O> {}
        unsafe impl<I: Data, O: Data> Finalize for Function<I, O> {}
    };
}

/// Call a function with a given input.
#[macro_export]
macro_rules! call {
    ($fun:expr, $args:expr, $ctx:expr) => {
        ($fun.ptr)($args, $fun.env, $ctx)
    };
}

#[macro_export]
macro_rules! direct_call {
    ($fun:ident, $args:expr, $ctx:expr) => {
        $fun($args, ErasablePtr::erase(empty_env()), $ctx)
    };
}

/// Create a function with an environment.
#[macro_export]
macro_rules! function {
    ($fun:ident, $env:expr) => {
        Function {
            ptr: $fun,
            tag: FunctionTag::$fun,
            env: ErasablePtr::erase(Box::new($env)),
        }
    };
    ($fun:ident) => {
        Function {
            ptr: $fun,
            tag: FunctionTag::$fun,
            env: ErasablePtr::erase(empty_env()),
        }
    };
}

#[macro_export]
macro_rules! function_type {
    ($input:ty, $output:ty) => {
        Function<$input, Tag, $output>
    };
}

pub fn empty_env() -> NonNull<()> {
    NonNull::from(&())
}
