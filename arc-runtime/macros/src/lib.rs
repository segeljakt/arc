#![feature(proc_macro_span)]

use proc_macro as pm;
use proc_macro::TokenStream;
use proc_macro2 as pm2;

mod derives;
mod enums;
mod functions;
mod main_function;
mod nonpersistent_tasks;
mod persistent_tasks;
mod structs;

#[proc_macro_derive(Abstract)]
pub fn derive_abstract(input: TokenStream) -> TokenStream {
    derives::derive_abstract(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(Collectable)]
pub fn derive_collectable(input: TokenStream) -> TokenStream {
    derives::derive_collectable(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(Finalize)]
pub fn derive_finalize(input: TokenStream) -> TokenStream {
    derives::derive_finalize(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(NoTrace)]
pub fn derive_notrace(input: TokenStream) -> TokenStream {
    derives::derive_notrace(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(Trace)]
pub fn derive_trace(input: TokenStream) -> TokenStream {
    derives::derive_trace(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(Alloc)]
pub fn derive_alloc(input: TokenStream) -> TokenStream {
    derives::derive_alloc(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(Send)]
pub fn derive_send(input: TokenStream) -> TokenStream {
    derives::derive_send(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(Sync)]
pub fn derive_sync(input: TokenStream) -> TokenStream {
    derives::derive_sync(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[proc_macro_derive(Unpin)]
pub fn derive_unpin(input: TokenStream) -> TokenStream {
    derives::derive_unpin(syn::parse_macro_input!(input as syn::DeriveInput))
}

#[cfg(legacy)]
#[proc_macro]
pub fn call(input: TokenStream) -> TokenStream {
    input
}

#[cfg(not(legacy))]
#[proc_macro]
pub fn call(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::Expr);
    match input {
        syn::Expr::Call(e) => {
            let func = e.func;
            let args = e.args;
            match args.len() {
                1 => quote::quote!(#func((#args,), ctx)).into(),
                _ => quote::quote!(#func((#args), ctx)).into(),
            }
        }
        _ => panic!("Expected function call expression"),
    }
}

#[cfg(legacy)]
#[proc_macro]
pub fn call_indirect(input: TokenStream) -> TokenStream {
    input
}

#[cfg(not(legacy))]
#[proc_macro]
pub fn call_indirect(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::Expr);
    match input {
        syn::Expr::Call(e) => {
            let func = e.func;
            let args = e.args;
            match args.len() {
                1 => quote::quote!((#func.ptr)((#args,), ctx)).into(),
                _ => quote::quote!((#func.ptr)((#args), ctx)).into(),
            }
        }
        syn::Expr::Macro(mut e) => {
            e.mac.tokens.extend([quote::quote!(,), quote::quote!(ctx)]);
            quote::quote!(#e).into()
        }
        _ => panic!("Expected function or macro call expression"),
    }
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
#[cfg(not(legacy))]
#[proc_macro]
pub fn enwrap(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut path: syn::Path = parse(&mut iter);
    concrete_enum_path(&mut path);
    let data: syn::Expr = parse(&mut iter);
    quote::quote!(#path(#data).alloc(ctx)).into()
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
#[cfg(not(legacy))]
#[proc_macro]
pub fn is(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut path: syn::Path = parse(&mut iter);
    concrete_enum_path(&mut path);
    let data: syn::Expr = parse(&mut iter);
    quote::quote!(if let #path(_) = #data.0.as_ref() { true } else { false }).into()
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
#[cfg(not(legacy))]
#[proc_macro]
pub fn unwrap(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut path: syn::Path = parse(&mut iter);
    concrete_enum_path(&mut path);
    let expr: syn::Expr = parse(&mut iter);
    quote::quote!(if let #path(v) = &*#expr.0 { v.clone() } else { unreachable!() }).into()
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
#[cfg(not(legacy))]
#[proc_macro]
pub fn new(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut data: syn::ExprStruct = parse(&mut iter);
    concrete_struct_path(&mut data.path);
    quote::quote!((#data).alloc(ctx)).into()
}

#[cfg(not(legacy))]
#[proc_macro]
pub fn vector(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let data: Vec<syn::Expr> = parse_all(&mut iter);
    quote::quote!(_vector!([#(#data),*], ctx)).into()
}

#[proc_macro]
pub fn erase(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let expr: syn::Expr = parse(&mut iter);
    let ident: syn::Ident = parse(&mut iter);
    let (wrapper_impl, wrapper_cons) = generate_wrapper(&ident);
    let wrapper = wrapper_cons(expr);
    quote::quote!(
        {
            #wrapper_impl
            Erased::erase(#wrapper, ctx)
        }
    )
    .into()
}

#[proc_macro]
pub fn unerase(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let expr: syn::Expr = parse(&mut iter);
    let id: syn::Ident = parse(&mut iter);
    let (wrapper_impl, _) = generate_wrapper(&id);
    quote::quote!(
        {
            #wrapper_impl
            Erased::unerase::<#id>(#expr, ctx)
        }
    )
    .into()
}

fn generate_wrapper(id: &syn::Ident) -> (pm2::TokenStream, impl Fn(syn::Expr) -> pm2::TokenStream) {
    let span = id.span().unwrap().start();
    let line = span.line;
    let column = span.column;
    let abstract_id: syn::Ident = new_id(format!("Wrapper_{}_{}", line, column));
    let concrete_id: syn::Ident = new_id(format!("ConcreteWrapper_{}_{}", line, column));
    let sharable_wrapper_mod_id = new_id(format!("sharable_{}", abstract_id));
    let sendable_wrapper_mod_id = new_id(format!("sendable_{}", abstract_id));
    let wrapper_impl = quote::quote!(
        mod #sharable_wrapper_mod_id {
            use arc_runtime::prelude::*;
            #[derive(Clone, Debug, Send, Sync, Unpin, From, Deref, Abstract, Collectable, Finalize, Trace)]
            #[repr(transparent)]
            pub struct #abstract_id(pub #concrete_id);
            #[derive(Clone, Debug, Collectable, Finalize, Trace)]
            #[repr(transparent)]
            pub struct #concrete_id(pub super::#id);
        }

        mod #sendable_wrapper_mod_id {
            use arc_runtime::prelude::*;
            #[derive(Debug, Deref, From, Abstract)]
            #[repr(transparent)]
            pub struct #abstract_id(pub #concrete_id);
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct #concrete_id(pub <super::#id as DynSharable>::T);
        }

        impl DynSharable for #sharable_wrapper_mod_id::#abstract_id {
            type T = <Erased as DynSharable>::T;
            fn into_sendable(&self, ctx: &mut Context) -> Self::T {
                Self::T::erase(#sendable_wrapper_mod_id::#abstract_id(#sendable_wrapper_mod_id::#concrete_id(self.0.0.into_sendable(ctx))), ctx)
            }
        }

        impl DynSendable for #sendable_wrapper_mod_id::#abstract_id {
            type T = Erased;
            fn into_sharable(&self, ctx: &mut Context) -> Self::T {
                Self::T::erase(#sharable_wrapper_mod_id::#abstract_id(#sharable_wrapper_mod_id::#concrete_id(self.0.0.into_sharable(ctx))), ctx)
            }
        }
    );
    let wrapper_cons = move |expr| quote::quote!(#sharable_wrapper_mod_id::#abstract_id(#sharable_wrapper_mod_id::#concrete_id(#expr)));
    (wrapper_impl, wrapper_cons)
}

fn concrete_enum_path(path: &mut syn::Path) {
    let mut x = path.segments.iter_mut();
    match (x.next(), x.next(), x.next()) {
        (Some(_), Some(i), Some(_)) => i.ident = new_id(format!("Concrete{}", i.ident)),
        (Some(i), Some(_), None) => i.ident = new_id(format!("Concrete{}", i.ident)),
        (Some(_), None, None) => {}
        _ => unreachable!(),
    }
}

fn concrete_struct_path(path: &mut syn::Path) {
    let mut x = path.segments.iter_mut();
    match (x.next(), x.next()) {
        (Some(_), Some(i)) => i.ident = new_id(format!("Concrete{}", i.ident)),
        (Some(i), None) => i.ident = new_id(format!("Concrete{}", i.ident)),
        _ => unreachable!(),
    }
}

fn parse<T: syn::parse::Parse>(input: &mut impl Iterator<Item = pm::TokenTree>) -> T {
    let mut stream = pm::TokenStream::new();
    while let Some(token) = input.next() {
        match token {
            pm::TokenTree::Punct(t) if t.as_char() == ',' => break,
            _ => stream.extend([token]),
        }
    }
    syn::parse::<T>(stream).unwrap()
}

fn parse_all<T: syn::parse::Parse>(input: &mut impl Iterator<Item = pm::TokenTree>) -> Vec<T> {
    let mut nodes = Vec::new();
    let mut stream = pm::TokenStream::new();
    while let Some(token) = input.next() {
        match token {
            pm::TokenTree::Punct(t) if t.as_char() == ',' => {
                nodes.push(syn::parse::<T>(stream).unwrap());
                stream = pm::TokenStream::new();
            }
            _ => stream.extend([token]),
        }
    }
    nodes
}

/// Declares a new enum which is compatible with the `codegen::{enwrap, unwrap, is}` API.
///
/// Any expansion of the macro satisfies the following properties:
/// * Enums:
///   * Each enum variants is imported into the global namespace.
/// * Structs
/// * Tasks
#[proc_macro_attribute]
pub fn rewrite(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as syn::Item);
    let attr = syn::parse_macro_input!(attr as syn::AttributeArgs);
    match item {
        syn::Item::Enum(item) => enums::rewrite(attr, item),
        syn::Item::Struct(item) => structs::rewrite(attr, item),
        syn::Item::Mod(item) if has_meta_key("nonpersistent", &get_metas(&attr)) => {
            nonpersistent_tasks::rewrite(attr, item)
        }
        syn::Item::Mod(item) if has_meta_key("persistent", &get_metas(&attr)) => {
            persistent_tasks::rewrite(attr, item)
        }
        syn::Item::Fn(item) if has_meta_key("main", &get_metas(&attr)) => {
            main_function::rewrite(attr, item)
        }
        syn::Item::Fn(item) => functions::rewrite(attr, item),
        _ => panic!(
            "{}",
            format!(
                r#"[rewrite] expects enum or struct as input, e.g., \
                   * `#[rewrite] enum MyEnum ...` \
                   * `#[rewrite] struct MyStruct ...` \
                   * `#[rewrite(async)] mod MyTask ...`"#
            )
        ),
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
