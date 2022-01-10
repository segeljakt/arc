use crate::new_id;
use proc_macro::TokenStream;

pub fn derive_abstract(input: syn::DeriveInput) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let abstract_ident = &input.ident;
    let concrete_ident = new_id(format!("Concrete{}", abstract_ident));
    quote::quote!(
        impl #impl_generics Abstract for #abstract_ident #type_generics #where_clause {
            type Concrete = #concrete_ident #type_generics;
        }
        impl #impl_generics Concrete for #concrete_ident #type_generics #where_clause {
            type Abstract = #abstract_ident #type_generics;
        }
    )
    .into()
}

pub fn derive_collectable(input: syn::DeriveInput) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let name = &input.ident;
    quote::quote!(impl #impl_generics Collectable #where_clause for #name #type_generics {}).into()
}

pub fn derive_finalize(input: syn::DeriveInput) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let name = &input.ident;
    quote::quote!(unsafe impl #impl_generics Finalize for #name #type_generics #where_clause {})
        .into()
}

pub fn derive_notrace(input: syn::DeriveInput) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let name = &input.ident;
    quote::quote!(
        unsafe impl #impl_generics Trace for #name #type_generics #where_clause {
            fn trace(&mut self, vis: &mut dyn Visitor) { }
        }
    )
    .into()
}

pub fn derive_trace(input: syn::DeriveInput) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let name = &input.ident;
    match &input.data {
        syn::Data::Struct(data) => {
            let field = data.fields.iter().enumerate().map(|(index, field)| {
                field
                    .ident
                    .as_ref()
                    .map(|ident| quote::quote!(#ident))
                    .unwrap_or_else(|| {
                        let index = syn::Index::from(index);
                        quote::quote!(#index)
                    })
            });
            quote::quote!(
                unsafe impl #impl_generics Trace for #name #type_generics #where_clause {
                    fn trace(&mut self, vis: &mut dyn Visitor) {
                        #(self.#field.trace(vis));*
                    }
                }
            )
            .into()
        }
        syn::Data::Enum(data) => {
            let variant = data.variants.iter().map(|v| &v.ident);
            quote::quote!(
                unsafe impl #impl_generics Trace for #name #type_generics #where_clause {
                    fn trace(&mut self, vis: &mut dyn Visitor) {
                        match self {
                            #(Self::#variant(data) => data.trace(vis),)*
                        }
                    }
                }
            )
            .into()
        }
        syn::Data::Union(_) => unreachable!(),
    }
}

pub fn derive_alloc(input: syn::DeriveInput) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let abstract_id = &input.ident;
    let concrete_id = new_id(format!("Concrete{}", abstract_id));
    quote::quote!(
        impl #impl_generics Alloc<#abstract_id> for #concrete_id #type_generics #where_clause {
            fn alloc(self, ctx: &mut Context) -> #abstract_id #type_generics {
                #abstract_id(ctx.mutator.allocate(self, AllocationSpace::New))
            }
        }
    )
    .into()
}

pub fn derive_send(input: syn::DeriveInput) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let name = &input.ident;
    quote::quote!(unsafe impl #impl_generics Send for #name #type_generics #where_clause {}).into()
}

pub fn derive_sync(input: syn::DeriveInput) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let name = &input.ident;
    quote::quote!(unsafe impl #impl_generics Sync for #name #type_generics #where_clause {}).into()
}

pub fn derive_unpin(input: syn::DeriveInput) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let name = &input.ident;
    quote::quote!(impl #impl_generics Unpin for #name #type_generics #where_clause {}).into()
}
