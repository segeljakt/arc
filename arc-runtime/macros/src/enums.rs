//! Codegen for enums

use proc_macro as pm;
use quote::quote;

use crate::new_id;

#[allow(unused)]
pub(crate) fn rewrite(_: syn::AttributeArgs, mut enum_item: syn::ItemEnum) -> pm::TokenStream {
    let abstract_id = enum_item.ident.clone();
    let concrete_id = new_id(format!("Concrete{}", abstract_id));
    let sharable_mod_id = new_id(format!("sharable_enum_{}", abstract_id));
    let sendable_mod_id = new_id(format!("sendable_enum_{}", abstract_id));

    let mut concrete_sharable_enum_item = enum_item.clone();
    let mut concrete_sendable_enum_item = enum_item;

    concrete_sharable_enum_item.ident = concrete_id.clone();
    concrete_sendable_enum_item.ident = concrete_id.clone();

    concrete_sharable_enum_item
        .variants
        .iter_mut()
        .for_each(|v| {
            v.fields.iter_mut().for_each(|f| {
                let ty = f.ty.clone();
                f.ty = syn::parse_quote!(super::#ty);
            })
        });

    concrete_sendable_enum_item
        .variants
        .iter_mut()
        .for_each(|v| {
            v.fields.iter_mut().for_each(|f| {
                let ty = f.ty.clone();
                f.ty = syn::parse_quote!(<super::#ty as arc_runtime::prelude::IntoSendable>::T);
            })
        });

    let variant_id = concrete_sharable_enum_item
        .variants
        .iter()
        .map(|v| &v.ident)
        .collect::<Vec<_>>();

    quote!(

        pub mod #sharable_mod_id {
            use arc_runtime::prelude::*;

            #[derive(Clone, Debug)]
            pub struct #abstract_id(pub std::rc::Rc<#concrete_id>);

            impl From<#concrete_id> for #abstract_id {
                fn from(v: #concrete_id) -> Self {
                    Self(std::rc::Rc::new(v))
                }
            }

            #[derive(Clone, Debug)]
            #concrete_sharable_enum_item
        }

        pub mod #sendable_mod_id {
            use arc_runtime::prelude::*;

            #[derive(Clone, Debug)]
            pub struct #abstract_id(pub Box<#concrete_id>);

            impl From<#concrete_id> for #abstract_id {
                fn from(v: #concrete_id) -> Self {
                    Self(Box::new(v))
                }
            }

            #[derive(Clone, Debug)]
            #concrete_sendable_enum_item
        }

        use #sharable_mod_id::#abstract_id;
        use #sharable_mod_id::#concrete_id::*;

        impl IntoSendable for #sharable_mod_id::#abstract_id {
            type T = #sendable_mod_id::#abstract_id;
            fn into_sendable(self) -> Self::T {
                match self.0.as_ref() {
                    #(
                        #sharable_mod_id::#concrete_id::#variant_id(x) =>
                        #sendable_mod_id::#concrete_id::#variant_id(x.clone().into_sendable()).into()
                    ),*
                }
            }
        }

        impl IntoSharable for #sendable_mod_id::#abstract_id {
            type T = #sharable_mod_id::#abstract_id;
            fn into_sharable(self) -> Self::T {
                match *self.0 {
                    #(
                        #sendable_mod_id::#concrete_id::#variant_id(x) =>
                        #sharable_mod_id::#concrete_id::#variant_id(x.into_sharable()).into()
                    ),*
                }
            }
        }

    )
    .into()
}
