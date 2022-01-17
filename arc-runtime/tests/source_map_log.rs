#![feature(arbitrary_self_types)]
#![allow(unused_mut)]

macro_rules! compile_test {
    {$($mod:tt)::+} => {
        use arc_runtime::prelude::*;

        // Note: This may only be used by the main thread.
        static EXECUTOR: Executor = Executor::new();

        #[derive(ComponentDefinition)]
        struct Source<I: Data, T: Data>
        where
            I: IntoIterator<Item = T>,
            <I as IntoIterator>::IntoIter: Data,
        {
            ctx: ComponentContext<Self>,
            iter: I,
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

        impl<I: Data, T: Data> Source<I, T>
        where
            I: IntoIterator<Item = T>,
            <I as IntoIterator>::IntoIter: Data,
        {
            fn new(iter: I, pushable: $($mod)::+::Pushable<T>) -> Self {
                Self {
                    ctx: ComponentContext::uninitialised(),
                    iter,
                    pushable,
                }
            }

            async fn run(mut self: ComponentDefinitionAccess<Self>) -> Control<()> {
                let i = self.iter.clone();
                for x in i {
                    self.pushable.push(x).await?;
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

        impl<I: Data, T: Data> ComponentLifecycle for Source<I, T>
        where
            I: IntoIterator<Item = T>,
            <I as IntoIterator>::IntoIter: Data,
        {
            fn on_start(&mut self) -> Handled {
                self.spawn_local(move |async_self| async move {
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

        impl<I: Data, T: Data> Actor for Source<I, T>
        where
            I: IntoIterator<Item = T>,
            <I as IntoIterator>::IntoIter: Data,
        {
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

        fn source<I: Data, T: Data>(i: I) -> $($mod)::+::Pullable<T>
        where
            I: IntoIterator<Item = T>,
            <I as IntoIterator>::IntoIter: Data,
        {
            let (o0, o1) = $($mod)::+::channel(&EXECUTOR);
            EXECUTOR.create_task(move || Source::new(i, o0));
            o1
        }

        fn map<A: Data, B: Data>(a: $($mod)::+::Pullable<A>, f: fn(A) -> B) -> $($mod)::+::Pullable<B> {
            let (b0, b1) = $($mod)::+::channel(&EXECUTOR);
            EXECUTOR.create_task(move || Map::new(a, f, b0));
            b1
        }

        fn log<T: Data>(a: $($mod)::+::Pullable<T>) {
            EXECUTOR.create_task(move || Log::new(a));
        }

        fn plus_one(x: i32) -> i32 {
            x + 1
        }

        #[test]
        fn main() {
            EXECUTOR.init(KompactConfig::default().build().unwrap());
            log(map(source(0..100), plus_one));
            EXECUTOR.await_termination();
        }
    }
}

// mod source_map_log_remote_concurrent {
//     compile_test!(arc_runtime::channels::remote::concurrent);
// }

// mod source_map_log_remote_broadcast {
//     compile_test!(arc_runtime::channels::remote::broadcast);
// }
//
mod source_map_log_local_concurrent {
    compile_test!(arc_runtime::channels::local::concurrent);
}
//
// mod source_map_log_local_broadcast {
//     compile_test!(arc_runtime::channels::local::broadcast);
// }
