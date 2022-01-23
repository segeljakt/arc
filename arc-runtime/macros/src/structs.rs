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
            f.ty = syn::parse_quote!(<super::#ty as IntoSendable>::T);
        });

    let field_id = concrete_sendable_struct_item
        .fields
        .iter()
        .map(|f| &f.ident)
        .collect::<Vec<_>>();

    quote!(

        use arc_runtime::prelude::*;
        pub mod #sharable_mod_id {
            use arc_runtime::prelude::*;

            #[derive(Clone, Debug, From, Deref)]
            pub struct #abstract_id(pub Gc<#concrete_id>);

            impl Alloc<#abstract_id> for #concrete_id {
                fn alloc(self, ctx: &mut Context) -> #abstract_id {
                    #abstract_id(ctx.mutator.allocate(self, AllocationSpace::New))
                }
            }

            #[derive(Clone, Debug)]
            #concrete_sharable_struct_item

            impl Collectable for #abstract_id {}
            unsafe impl Finalize for #abstract_id {}
            unsafe impl Trace for #abstract_id {
                fn trace(&mut self, vis: &mut dyn Visitor) {
                    self.0.trace(vis);
                }
            }

            impl Collectable for #concrete_id {}
            unsafe impl Finalize for #concrete_id {}
            unsafe impl Trace for #concrete_id {
                fn trace(&mut self, vis: &mut dyn Visitor) {
                    #(self.#field_id.trace(vis);)*
                }
            }
            unsafe impl Send for #abstract_id {}
            unsafe impl Sync for #abstract_id {}
            impl Unpin for #abstract_id {}
        }

        mod #sendable_mod_id {
            use arc_runtime::prelude::*;

            #[derive(Clone, Debug, Deref, From)]
            #[from(forward)]
            pub struct #abstract_id(pub Box<#concrete_id>);

            #[derive(Clone, Debug)]
            #concrete_sendable_struct_item
        }

        use #sharable_mod_id::#abstract_id;
        use #sharable_mod_id::#concrete_id;

        impl IntoSendable for #sharable_mod_id::#abstract_id {
            type T = #sendable_mod_id::#abstract_id;
            fn into_sendable(self, ctx: &mut Context) -> Self::T {
                #sendable_mod_id::#concrete_id {
                    #(#field_id: self.0.#field_id.clone().into_sendable(ctx)),*
                }.into()
            }
        }

        impl IntoSharable for #sendable_mod_id::#abstract_id {
            type T = #sharable_mod_id::#abstract_id;
            fn into_sharable(self, ctx: &mut Context) -> Self::T {
                #sharable_mod_id::#concrete_id {
                    #(#field_id: self.0.#field_id.into_sharable(ctx)),*
                }.alloc(ctx)
            }
        }
    )
    .into()
}
