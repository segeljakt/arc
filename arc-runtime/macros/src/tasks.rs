#![allow(unused)]

use crate::new_id;

use proc_macro as pm;
use proc_macro2 as pm2;
use quote::quote;
use syn::parse::*;
use syn::punctuated::Punctuated;
use syn::token::Comma;

/// task id(a:Pullable[i32], b:Pullable[i32]): (c:Pushable[i32], d:Pushable[i32]) {
///     val x = receive a;
///     val y = receive b;
///     c ! x;
///     d ! y;
/// }
///
/// Becomes
///
/// ```no_run
/// #[rewrite]
/// mod my_task {
///     fn task(
///         a:Pullable<i32>,
///         b:Pullable<i32>,
///         #[output] c:Pushable<i32>,
///         #[output] d:Pushable<i32>
///     ) {
///         let x = pull!(a);
///         let y = pull!(b);
///         push!(c, x);
///         push!(d, y);
///     }
/// }
/// ```

pub(crate) fn rewrite(attr: syn::AttributeArgs, item: syn::ItemMod) -> pm::TokenStream {
    let task_name = item.ident.clone();

    let mod_name = new_id(format!("mod_{task_name}"));

    let items = item.content.expect("Expected module to contain items").1;

    let task = items
        .iter()
        .find_map(|item| match item {
            syn::Item::Fn(item) if item.sig.ident == "task" => Some(item),
            _ => None,
        })
        .expect(r#"Expected a function with name "task" in module"#);

    let task_body = task.block.clone();

    let (iparams, oparams): (Vec<_>, Vec<_>) =
        task.sig.inputs.clone().into_iter().partition(|p| match p {
            syn::FnArg::Receiver(_) => unreachable!(),
            syn::FnArg::Typed(p) => !has_attr_key("output", &p.attrs),
        });

    //     println!("INPUTS {:?}", iparams);
    //     println!("OUTPUTS {:?}", oparams);

    let (iparam_name, iparam_type): (Vec<_>, Vec<_>) = split_name_type(iparams);
    let (oparam_name, oparam_type): (Vec<_>, Vec<_>) = split_name_type(oparams);

    let oparam_pull_name = oparam_name
        .iter()
        .map(|name| new_id(format!("{name}_pull")))
        .collect::<Vec<_>>();

    let oparam_pull_type = oparam_type
        .iter()
        .map(|ty| quote!(<#ty as Channel>::Pullable))
        .collect::<Vec<_>>();

    //     println!("{:?}", oparam_pull_name);
    //     println!("{:?}", oparam_pull_type);

    quote!(
        use #mod_name::#task_name;
        #[allow(clippy::all)]
        #[allow(non_snake_case)]
        pub mod #mod_name {
            use arc_runtime::prelude::*;
            use arc_runtime::channels::local::concurrent::{Pushable, Pullable};
            use super::*;

            struct Task {
                pub ctx: ComponentContext<Self>,
                pub event_time: DateTime,
                #(pub #iparam_name: #iparam_type,)*
                #(pub #oparam_name: #oparam_type,)*
            }

            // Allows the component to be sent across threads even if it contains fields which are
            // not safely sendable. The code generator is required to generate thread-safe code.
            unsafe impl Send for Task {}

            pub fn #task_name(#(#iparam_name: #iparam_type,)*) -> (#(#oparam_pull_type,)*) {
                #(let (#oparam_name, #oparam_pull_name) = <#oparam_type as Channel>::channel(&EXECUTOR);)*
                EXECUTOR.create_task(move || Task::new(#(#iparam_name,)* #(#oparam_name,)*));
                (#(#oparam_pull_name,)*)
            }

            impl Task {
                #[allow(deprecated)] // NOTE: DateTime::unix_epoch is deprecated
                fn new(#(#iparam_name: #iparam_type,)* #(#oparam_name: #oparam_type,)*) -> Self {
                    Self {
                        ctx: ComponentContext::uninitialised(),
                        event_time: DateTime::unix_epoch(),
                        #(#iparam_name,)*
                        #(#oparam_name,)*
                    }
                }

                async fn run(async_self: ComponentDefinitionAccess<Self>) -> Control<()> {
                    #(let #iparam_name = async_self.#iparam_name.clone();)*
                    #(let #oparam_name = async_self.#oparam_name.clone();)*
                    #task_body
                    Control::Finished
                }
            }

            impl ComponentDefinition for Task {
                fn setup(&mut self, self_component: Arc<Component<Self>>) {
                    self.ctx.initialise(self_component.clone());
                }

                fn execute(&mut self, _max_events: usize, _skip: usize) -> ExecuteResult {
                    ExecuteResult::new(false, 0, 0)
                }

                fn ctx_mut(&mut self) -> &mut ComponentContext<Self> {
                    &mut self.ctx
                }

                fn ctx(&self) -> &ComponentContext<Self> {
                    &self.ctx
                }

                fn type_name() -> &'static str {
                    stringify!(#task_name)
                }
            }

            impl Actor for Task {
                type Message = TaskMessage;

                fn receive_local(&mut self, _: Self::Message) -> Handled {
                    todo!()
                }

                fn receive_network(&mut self, _: NetMessage) -> Handled {
                    todo!()
                }
            }

            impl ComponentLifecycle for Task {
                fn on_start(&mut self) -> Handled {
                    self.spawn_local(move |async_self| async move {
                        Self::run(async_self).await;
                        Handled::DieNow
                    });
                    Handled::Ok
                }
            }

            use std::any::Any;
            use std::any::TypeId;
            use std::sync::Arc;

            impl DynamicPortAccess for Task {
                fn get_provided_port_as_any(&mut self, _: TypeId) -> Option<&mut dyn Any> {
                    unreachable!();
                }

                fn get_required_port_as_any(&mut self, _: TypeId) -> Option<&mut dyn Any> {
                    unreachable!();
                }
            }
        }
    )
    .into()
}

fn poll_port(
    on_event: &pm2::TokenStream,
    treshold: pm2::TokenStream,
    skip: pm2::TokenStream,
    port: &syn::Ident,
) -> pm2::TokenStream {
    quote! {
        if skip <= #treshold {
            if count >= max_events {
                return ExecuteResult::new(false, count, #skip);
            }
        }
        if let Some(event) = self.#port.dequeue() {
            let res = #on_event;
            count += 1;
            done_work = true;
            if let Handled::BlockOn(blocking_future) = res {
                self.ctx_mut().set_blocking(blocking_future);
                return ExecuteResult::new(true, count, #skip);
            }
        }
    }
}

fn future(future: Option<syn::Type>) -> pm2::TokenStream {
    if let Some(ty) = future {
        quote! {}
    } else {
        quote! {}
    }
}

fn get_attr_val(name: &str, attr: &[syn::NestedMeta]) -> syn::Ident {
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
            syn::NestedMeta::Lit(lit) => None,
        })
        .unwrap_or_else(|| panic!("`{} = <id>` missing from identifiers", name))
}

fn has_attr_key(name: &str, attr: &[syn::Attribute]) -> bool {
    attr.iter()
        .any(|a| matches!(a.parse_meta(), Ok(syn::Meta::Path(x)) if x.is_ident(name)))
}

fn split_name_type(params: Vec<syn::FnArg>) -> (Vec<syn::Ident>, Vec<syn::Type>) {
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
