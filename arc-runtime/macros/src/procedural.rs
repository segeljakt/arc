use crate::new_id;

use proc_macro as pm;
use proc_macro::TokenStream;
use proc_macro2 as pm2;

#[cfg(feature = "legacy")]
pub fn call(input: syn::Expr) -> TokenStream {
    quote::quote!(#input).into()
}

#[cfg(not(feature = "legacy"))]
pub fn call(input: syn::Expr) -> TokenStream {
    match input {
        syn::Expr::Call(e) => {
            let func = e.func;
            let args = e.args;
            if args.len() == 1 && !args.trailing_punct() {
                quote::quote!(#func((#args,), ctx)).into()
            } else {
                quote::quote!(#func((#args), ctx)).into()
            }
        }
        _ => panic!("Expected function call expression"),
    }
}

#[cfg(feature = "legacy")]
pub fn call_indirect(input: syn::Expr) -> TokenStream {
    quote::quote!(#input).into()
}

#[cfg(not(feature = "legacy"))]
pub fn call_indirect(input: syn::Expr) -> TokenStream {
    match input {
        syn::Expr::Call(e) => {
            let func = e.func;
            let args = e.args;
            if args.len() == 1 && !args.trailing_punct() {
                quote::quote!((#func.ptr)((#args,), ctx)).into()
            } else {
                quote::quote!((#func.ptr)((#args), ctx)).into()
            }
        }
        _ => panic!("Expected function call expression"),
    }
}

#[cfg(feature = "legacy")]
pub fn enwrap(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut path: syn::Path = parse(&mut iter);
    concrete_enum_path(&mut path);
    let data: syn::Expr = parse(&mut iter);
    quote::quote!(#path(#data).into()).into()
}

#[cfg(not(feature = "legacy"))]
pub fn enwrap(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut path: syn::Path = parse(&mut iter);
    concrete_enum_path(&mut path);
    let data: syn::Expr = parse(&mut iter);
    quote::quote!(#path(#data).alloc(ctx)).into()
}

pub fn is(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut path: syn::Path = parse(&mut iter);
    concrete_enum_path(&mut path);
    let data: syn::Expr = parse(&mut iter);
    quote::quote!(if let #path(_) = #data.0.as_ref() { true } else { false }).into()
}

pub fn unwrap(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut path: syn::Path = parse(&mut iter);
    concrete_enum_path(&mut path);
    let expr: syn::Expr = parse(&mut iter);
    quote::quote!(if let #path(v) = &*#expr.0 { v.clone() } else { unreachable!() }).into()
}

#[cfg(feature = "legacy")]
pub fn new(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut data: syn::ExprStruct = parse(&mut iter);
    concrete_struct_path(&mut data.path);
    quote::quote!((#data).into()).into()
}

#[cfg(not(feature = "legacy"))]
pub fn new(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut data: syn::ExprStruct = parse(&mut iter);
    concrete_struct_path(&mut data.path);
    quote::quote!((#data).alloc(ctx)).into()
}

pub fn vector(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let data: Vec<syn::Expr> = parse_all(&mut iter);
    quote::quote!(_vector!([#(#data),*], ctx)).into()
}

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
