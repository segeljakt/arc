use proc_macro as pm;
use syn::visit_mut::VisitMut;

use crate::functions::Visitor;

pub(crate) fn rewrite(_attr: syn::AttributeArgs, mut item: syn::ItemFn) -> pm::TokenStream {
    Visitor::default().visit_item_fn_mut(&mut item);
    item.block.stmts.insert(
        0,
        syn::parse_quote! {
            let ctx = &mut Context::new();
        },
    );
    quote::quote!(#item).into()
}
