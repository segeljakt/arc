use proc_macro::TokenStream;
use proc_macro2 as pm2;

mod enums;
mod functions;
mod nonpersistent_tasks;
mod persistent_tasks;
mod structs;
mod main_function;

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
