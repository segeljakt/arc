#![feature(proc_macro_span)]

use proc_macro::TokenStream;
use proc_macro2 as pm2;

mod proc_macro_derives;
mod proc_macro_attrs {
    pub mod rewrite {
        pub mod driver;
        pub mod enums;
        pub mod externs;
        pub mod functions;
        pub mod nonpersistent_tasks;
        pub mod persistent_tasks;
        pub mod structs;
    }
}
mod proc_macros;

#[proc_macro_derive(Abstract)]
pub fn derive_abstract(input: TokenStream) -> TokenStream {
    proc_macro_derives::derive_abstract(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(Collectable)]
pub fn derive_collectable(input: TokenStream) -> TokenStream {
    proc_macro_derives::derive_collectable(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(Finalize)]
pub fn derive_finalize(input: TokenStream) -> TokenStream {
    proc_macro_derives::derive_finalize(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(NoTrace)]
pub fn derive_notrace(input: TokenStream) -> TokenStream {
    proc_macro_derives::derive_notrace(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(Trace)]
pub fn derive_trace(input: TokenStream) -> TokenStream {
    proc_macro_derives::derive_trace(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(Garbage)]
pub fn derive_garbage(input: TokenStream) -> TokenStream {
    proc_macro_derives::derive_garbage(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(Alloc)]
pub fn derive_alloc(input: TokenStream) -> TokenStream {
    proc_macro_derives::derive_alloc(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(Send)]
pub fn derive_send(input: TokenStream) -> TokenStream {
    proc_macro_derives::derive_send(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(Sync)]
pub fn derive_sync(input: TokenStream) -> TokenStream {
    proc_macro_derives::derive_sync(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(Unpin)]
pub fn derive_unpin(input: TokenStream) -> TokenStream {
    proc_macro_derives::derive_unpin(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(NoSerde)]
pub fn derive_noserde(input: TokenStream) -> TokenStream {
    proc_macro_derives::derive_noserde(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(NoDebug)]
pub fn derive_nodebug(input: TokenStream) -> TokenStream {
    proc_macro_derives::derive_nodebug(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro]
pub fn call(input: TokenStream) -> TokenStream {
    proc_macros::call(syn::parse_macro_input!(input as syn::Expr))
}

#[proc_macro]
pub fn call_indirect(input: TokenStream) -> TokenStream {
    proc_macros::call_indirect(syn::parse_macro_input!(input as syn::Expr))
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
#[proc_macro]
pub fn enwrap(input: TokenStream) -> TokenStream {
    proc_macros::enwrap(input)
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
#[proc_macro]
pub fn is(input: TokenStream) -> TokenStream {
    proc_macros::is(input)
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
#[proc_macro]
pub fn unwrap(input: TokenStream) -> TokenStream {
    proc_macros::unwrap(input)
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
#[proc_macro]
pub fn new(input: TokenStream) -> TokenStream {
    proc_macros::new(input)
}

#[proc_macro]
pub fn vector(input: TokenStream) -> TokenStream {
    proc_macros::vector(input)
}

#[proc_macro]
pub fn erase(input: TokenStream) -> TokenStream {
    proc_macros::erase(input)
}

#[proc_macro]
pub fn unerase(input: TokenStream) -> TokenStream {
    proc_macros::unerase(input)
}

#[proc_macro]
pub fn push(input: TokenStream) -> TokenStream {
    proc_macros::push(input)
}

#[proc_macro]
pub fn pull(input: TokenStream) -> TokenStream {
    proc_macros::pull(input)
}

#[proc_macro]
pub fn pull_transition(input: TokenStream) -> TokenStream {
    proc_macros::pull_transition(input)
}

#[proc_macro]
pub fn push_transition(input: TokenStream) -> TokenStream {
    proc_macros::push_transition(input)
}

#[proc_macro]
pub fn transition(input: TokenStream) -> TokenStream {
    proc_macros::transition(input)
}

#[proc_macro]
pub fn terminate(input: TokenStream) -> TokenStream {
    proc_macros::terminate(input)
}

#[proc_macro]
pub fn wait(input: TokenStream) -> TokenStream {
    proc_macros::wait(input)
}

#[proc_macro_attribute]
pub fn rewrite(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as syn::AttributeArgs);
    let item = syn::parse_macro_input!(input as syn::Item);
    match item {
        syn::Item::Enum(item) => proc_macro_attrs::rewrite::enums::rewrite(attr, item),
        syn::Item::Struct(item) => proc_macro_attrs::rewrite::structs::rewrite(attr, item),
        syn::Item::Fn(item) if has_meta_name_val("unmangled", &get_metas(&attr)) => {
            proc_macro_attrs::rewrite::externs::rewrite(attr, item)
        }
        syn::Item::Fn(item)
            if item.sig.asyncness.is_some() && has_meta_key("nonpersistent", &get_metas(&attr)) =>
        {
            proc_macro_attrs::rewrite::nonpersistent_tasks::rewrite(attr, item)
        }
        syn::Item::Mod(item) if has_meta_key("persistent", &get_metas(&attr)) => {
            proc_macro_attrs::rewrite::persistent_tasks::rewrite(attr, item)
        }
        syn::Item::Fn(item) if has_meta_key("main", &get_metas(&attr)) => {
            proc_macro_attrs::rewrite::driver::rewrite(attr, item)
        }
        syn::Item::Fn(item) => proc_macro_attrs::rewrite::functions::rewrite(attr, item),
        _ => panic!("#[rewrite] expects an enum, struct, function, or module as input."),
    }
}

pub(crate) fn new_id(s: impl ToString) -> syn::Ident {
    syn::Ident::new(&s.to_string(), pm2::Span::call_site())
}

pub(crate) fn get_metas(attr: &[syn::NestedMeta]) -> Vec<syn::Meta> {
    attr.into_iter()
        .filter_map(|a| match a {
            syn::NestedMeta::Meta(m) => Some(m.clone()),
            _ => None,
        })
        .collect()
}

pub(crate) fn has_attr_key(name: &str, attr: &[syn::Attribute]) -> bool {
    attr.iter()
        .any(|a| matches!(a.parse_meta(), Ok(syn::Meta::Path(x)) if x.is_ident(name)))
}

pub(crate) fn has_meta_key(name: &str, meta: &[syn::Meta]) -> bool {
    meta.iter()
        .any(|m| matches!(m, syn::Meta::Path(x) if x.is_ident(name)))
}

pub(crate) fn has_meta_name_val(name: &str, meta: &[syn::Meta]) -> bool {
    meta.iter()
        .any(|m| matches!(m, syn::Meta::NameValue(x) if x.path.is_ident(name)))
}

pub(crate) fn _has_nested_meta_key(name: &str, meta: &[syn::NestedMeta]) -> bool {
    meta.iter().any(|m| match m {
        syn::NestedMeta::Meta(syn::Meta::Path(x)) if x.is_ident(name) => true,
        _ => false,
    })
}

#[allow(unused)]
pub(crate) fn get_attr_val(name: &str, attr: &[syn::NestedMeta]) -> syn::Ident {
    attr.iter()
        .find_map(|arg| match arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(nv) if nv.path.is_ident(name) => match &nv.lit {
                    syn::Lit::Str(x) => {
                        Some(x.parse().expect("Expected attr value to be an identifier"))
                    }
                    _ => None,
                },
                _ => None,
            },
            syn::NestedMeta::Lit(_) => None,
        })
        .unwrap_or_else(|| panic!("`{} = <id>` missing from identifiers", name))
}

pub(crate) fn split_name_type(params: Vec<syn::FnArg>) -> (Vec<syn::Ident>, Vec<syn::Type>) {
    params
        .into_iter()
        .map(|p| match p {
            syn::FnArg::Receiver(_) => unreachable!(),
            syn::FnArg::Typed(p) => match *p.pat {
                syn::Pat::Ident(i) => (i.ident, *p.ty),
                _ => unreachable!(),
            },
        })
        .unzip()
}
