use proc_macro as pm;
use quote::quote;

use crate::new_id;

#[allow(unused)]
pub(crate) fn rewrite(_: syn::AttributeArgs, struct_item: syn::ItemStruct) -> pm::TokenStream {
    let abstract_id = struct_item.ident.clone();
    let concrete_id = new_id(format!("Concrete{}", struct_item.ident));
    let sharable_mod_id = new_id(format!("sharable_struct_{}", struct_item.ident));
    let sendable_mod_id = new_id(format!("sendable_struct_{}", struct_item.ident));

    let mut concrete_sharable_struct_item = struct_item.clone();
    let mut concrete_sendable_struct_item = struct_item;

    concrete_sharable_struct_item.ident = concrete_id.clone();
    concrete_sendable_struct_item.ident = concrete_id.clone();

    // Generate the sendable struct
    concrete_sharable_struct_item
        .fields
        .iter_mut()
        .for_each(|f| {
            let ty = f.ty.clone();
            f.ty = syn::parse_quote!(super::#ty);
        });

    // Generate the sendable struct
    concrete_sendable_struct_item
        .fields
        .iter_mut()
        .for_each(|f| {
            let ty = f.ty.clone();
            f.ty = syn::parse_quote!(<super::#ty as arc_codegen::IntoSendable>::T);
        });

    let field_id = concrete_sendable_struct_item
        .fields
        .iter()
        .map(|f| &f.ident)
        .collect::<Vec<_>>();

    quote!(

        pub mod #sharable_mod_id {
            #[derive(Clone, Debug, arc_codegen::derive_more::Deref)]
            pub struct #abstract_id(pub std::rc::Rc<#concrete_id>);

            impl From<#concrete_id> for #abstract_id {
                fn from(v: #concrete_id) -> Self {
                    Self(std::rc::Rc::new(v))
                }
            }

            #[derive(Clone, Debug)]
            #concrete_sharable_struct_item
        }

        mod #sendable_mod_id {

            #[derive(Clone, Debug, arc_codegen::derive_more::Deref)]
            pub struct #abstract_id(pub Box<#concrete_id>);

            // Concrete to Abstract
            impl From<#concrete_id> for #abstract_id {
                fn from(v: #concrete_id) -> Self {
                    Self(Box::new(v))
                }
            }

            #[derive(Clone, Debug)]
            #concrete_sendable_struct_item
        }

        use #sharable_mod_id::{#abstract_id, #concrete_id};

        impl arc_codegen::IntoSendable for #sharable_mod_id::#abstract_id {
            type T = #sendable_mod_id::#abstract_id;
            fn into_sendable(self) -> Self::T {
                #sendable_mod_id::#concrete_id {
                    #(#field_id: self.0.as_ref().#field_id.clone().into_sendable()),*
                }.into()
            }
        }

        impl arc_codegen::IntoSharable for #sendable_mod_id::#abstract_id {
            type T = #sharable_mod_id::#abstract_id;
            fn into_sharable(self) -> Self::T {
                #sharable_mod_id::#concrete_id {
                    #(#field_id: self.0.#field_id.into_sharable()),*
                }.into()
            }
        }
    )
    .into()
}
