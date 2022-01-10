#![allow(unused)]

use crate::has_attr_key;
use crate::new_id;
use crate::split_name_type;

use proc_macro as pm;
use proc_macro2 as pm2;
use quote::quote;
use syn::parse::*;
use syn::punctuated::Punctuated;
use syn::token::Comma;

/// ```no_run
/// task id(a:Pullable[i32], b:Pullable[i32]): (c:Pushable[i32], d:Pushable[i32]) {
///     val x = receive a;
///     val y = receive b;
///     c ! x;
///     d ! y;
/// }
/// ```
///
/// Becomes
///
/// ```no_run
/// #[rewrite(impersistent)]
/// mod my_task {
///     fn task(a:Pullable<i32>, #[output] b:Pushable<i32>) {
///         let x = pull!(a);
///         push!(b, x);
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

    quote!(
        use #mod_name::#task_name;
        #[allow(clippy::all)]
        #[allow(non_snake_case)]
        #[allow(unreachable_code)]
        pub mod #mod_name {
            use arc_runtime::prelude::*;
            use arc_runtime::channels::local::task_parallel::{Pushable, Pullable};
            use super::*;

            #[derive(Send)]
            struct Task {
                pub ctx: ComponentContext<Self>,
                pub event_time: DateTime,
                #(pub #iparam_name: #iparam_type,)*
                #(pub #oparam_name: #oparam_type,)*
            }

            #[allow(unused_parens)]
            pub fn #task_name((#(#iparam_name,)*): (#(#iparam_type,)*), ctx: &mut Context) -> (#(#oparam_pull_type),*) {
                #(let #iparam_name = #iparam_name.into_sendable(ctx);)*
                #(let #iparam_name = #iparam_name.into_sharable(ctx);)*
                #(let (#oparam_name, #oparam_pull_name) = <#oparam_type as Channel>::channel(&ctx.component.system());)*
                ctx.launch(move || Task::new(#(#iparam_name,)* #(#oparam_name,)*));
                (#(#oparam_pull_name),*)
            }

            impl Task {
                #[allow(deprecated)] // NOTE: DateTime::unix_epoch is deprecated
                fn new(#(#iparam_name: #iparam_type,)* #(#oparam_name: #oparam_type,)*) -> Self {
                    Self {
                        ctx: ComponentContext::uninitialised(),
                        event_time: DateTime::new(date!(1970-01-01), time!(0:0:0)),
                        #(#iparam_name,)*
                        #(#oparam_name,)*
                    }
                }

                async fn run(#(#iparam_name: #iparam_type,)* #(#oparam_name: #oparam_type,) *ctx: &mut Context) -> Control<()> {
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
                    Handled::Ok
                }

                fn receive_network(&mut self, _: NetMessage) -> Handled {
                    todo!()
                }
            }

            impl ComponentLifecycle for Task {
                fn on_start(&mut self) -> Handled {
                    self.spawn_local(move |async_self| async move {
                        let component = async_self.ctx().component();
                        let mutator = instantiate_minimark(MiniMarkOptions::default());
                        let ctx = &mut Context::new(component, mutator);
                        #(let #iparam_name = async_self.#iparam_name.clone();)*
                        #(let #oparam_name = async_self.#oparam_name.clone();)*
                        Task::run(#(#iparam_name,)* #(#oparam_name,)* ctx).await;
                        Handled::DieNow
                    });
                    Handled::Ok
                }
            }

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
