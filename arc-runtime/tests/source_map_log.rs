#![feature(arbitrary_self_types)]
#![allow(unused_mut)]

macro_rules! compile_test {
    {$($mod:tt)::+} => {
        use arc_runtime::prelude::*;

        #[derive(ComponentDefinition)]
        struct Source<T: Data> {
            ctx: ComponentContext<Self>,
            vec: Vec<T>,
            pushable: $($mod)::+::Pushable<T>,
        }

        #[derive(ComponentDefinition)]
        struct Map<A: Data, B: Data> {
            ctx: ComponentContext<Self>,
            pullable: $($mod)::+::Pullable<A>,
            fun: fn(A) -> B,
            pushable: $($mod)::+::Pushable<B>,
        }

        #[derive(ComponentDefinition)]
        struct Log<T: Data> {
            ctx: ComponentContext<Self>,
            pullable: $($mod)::+::Pullable<T>,
        }

        impl<T: Data> Source<T> {
            fn new(vec: Vec<T>, pushable: $($mod)::+::Pushable<T>) -> Self {
                Self {
                    ctx: ComponentContext::uninitialised(),
                    vec,
                    pushable,
                }
            }

            async fn run(mut self: ComponentDefinitionAccess<Self>, ctx: &mut Context) -> Control<()> {
                let i = self.vec.clone();
                for x in 0..i.clone().len(ctx) {
                    let j = i.clone();
                    let v = j.at(x, ctx);
                    self.pushable.push(v.clone()).await?;
                }
                Control::Finished
            }
        }

        impl<A: Data, B: Data> Map<A, B> {
            fn new(pullable: $($mod)::+::Pullable<A>, f: fn(A) -> B, pushable: $($mod)::+::Pushable<B>) -> Self {
                Self {
                    ctx: ComponentContext::uninitialised(),
                    pullable,
                    fun: f,
                    pushable,
                }
            }

            async fn run(mut self: ComponentDefinitionAccess<Self>) -> Control<()> {
                let f = self.fun;
                loop {
                    let data = self.pullable.pull().await?;
                    self.pushable.push(f(data)).await?;
                }
            }
        }

        impl<T: Data> Log<T> {
            fn new(pullable: $($mod)::+::Pullable<T>) -> Self {
                Self {
                    ctx: ComponentContext::uninitialised(),
                    pullable,
                }
            }

            async fn run(mut self: ComponentDefinitionAccess<Self>) -> Control<()> {
                loop {
                    let data = self.pullable.pull().await?;
                    info!(self.log(), "Logging {:?}", data);
                }
            }
        }

        impl<T: Data> ComponentLifecycle for Source<T> {
            fn on_start(&mut self) -> Handled {
                self.spawn_local(move |async_self| async move {
                    let ctx = Context::new();
                    async_self.run().await;
                    Handled::DieNow
                });
                Handled::Ok
            }
        }

        impl<A: Data, B: Data> ComponentLifecycle for Map<A, B> {
            fn on_start(&mut self) -> Handled {
                self.spawn_local(move |async_self| async move {
                    async_self.run().await;
                    Handled::DieNow
                });
                Handled::Ok
            }
        }

        impl<T: Data> ComponentLifecycle for Log<T> {
            fn on_start(&mut self) -> Handled {
                self.spawn_local(move |async_self| async move {
                    async_self.run().await;
                    Handled::DieNow
                });
                Handled::Ok
            }
        }

        impl<T: Data> Actor for Source<T> {
            type Message = TaskMessage;

            fn receive_local(&mut self, _msg: Self::Message) -> Handled {
                Handled::Ok
            }

            fn receive_network(&mut self, _msg: NetMessage) -> Handled {
                unreachable!()
            }
        }

        impl<A: Data, B: Data> Actor for Map<A, B> {
            type Message = TaskMessage;

            fn receive_local(&mut self, _msg: Self::Message) -> Handled {
                Handled::Ok
            }

            fn receive_network(&mut self, _msg: NetMessage) -> Handled {
                unreachable!()
            }
        }

        impl<T: Data> Actor for Log<T> {
            type Message = TaskMessage;

            fn receive_local(&mut self, _msg: Self::Message) -> Handled {
                Handled::Ok
            }

            fn receive_network(&mut self, _msg: NetMessage) -> Handled {
                unreachable!()
            }
        }

        fn source<T: Data>(vec: Vec<T>, ctx: &mut Context) -> $($mod)::+::Pullable<T> {
            let (o0, o1) = $($mod)::+::channel(&ctx.system);
            let c = ctx.system.create(move || Source::new(vec, o0));
            ctx.system.start(&c);
            o1
        }

        fn map<A: Data, B: Data>(a: $($mod)::+::Pullable<A>, f: fn(A) -> B, ctx: &mut Context) -> $($mod)::+::Pullable<B> {
            let (b0, b1) = $($mod)::+::channel(&ctx.system);
            let c = ctx.system.create(move || Map::new(a, f, b0));
            ctx.system.start(&c);
            b1
        }

        fn log<T: Data>(a: $($mod)::+::Pullable<T>, ctx: &mut Context) {
            let c = ctx.system.create(move || Log::new(a));
            ctx.system.start(&c);
        }

        fn plus_one(x: i32) -> i32 {
            x + 1
        }

//         #[rewrite(main)]
        #[test]
        fn main() {
            let ctx = &mut Context::new();
            log(map(source(vector!([1, 2, 3], ctx), ctx), plus_one, ctx), ctx);
        }
    }
}

// mod source_map_log_remote_concurrent {
//     compile_test!(arc_runtime::channels::remote::concurrent);
// }

// mod source_map_log_remote_broadcast {
//     compile_test!(arc_runtime::channels::remote::broadcast);
// }

mod source_map_log_local_concurrent {
    compile_test!(arc_runtime::channels::local::task_parallel);
}

// mod source_map_log_local_broadcast {
//     compile_test!(arc_runtime::channels::local::broadcast);
// }
