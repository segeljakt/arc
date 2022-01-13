#![feature(arbitrary_self_types)]
#![allow(unused_mut)]

macro_rules! compile_test {
    {$($mod:tt)::+} => {
        use arc_runtime::prelude::*;

        // Note: This may only be used by the main thread.
        static EXECUTOR: Executor = Executor::new();

        #[derive(Actor, ComponentDefinition)]
        struct Source<I: IntoIterator<Item = T> + Data, T: Data>
        where
            <I as IntoIterator>::IntoIter: Data,
        {
            ctx: ComponentContext<Self>,
            iter: I,
            pushable: $($mod)::+::Pushable<T>,
        }

        #[derive(Actor, ComponentDefinition)]
        struct Map<A: Data, B: Data> {
            ctx: ComponentContext<Self>,
            pullable: $($mod)::+::Pullable<A>,
            fun: fn(A) -> B,
            pushable: $($mod)::+::Pushable<B>,
        }

        #[derive(Actor, ComponentDefinition)]
        struct Log<T: Data> {
            ctx: ComponentContext<Self>,
            pullable: $($mod)::+::Pullable<T>,
        }

        impl<I: IntoIterator<Item = T> + Data, T: Data> Source<I, T>
        where
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
                    info!(self.log(), "{:?}", data);
                }
            }
        }

        impl<I: IntoIterator<Item = T> + Data, T: Data> ComponentLifecycle for Source<I, T>
        where
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

        fn source<I, T>(i: I) -> $($mod)::+::Pullable<T>
        where
            I: IntoIterator<Item = T> + Data,
            T: Data,
            <I as IntoIterator>::IntoIter: Data,
        {
            let (a0, a1) = $($mod)::+::channel(&EXECUTOR);
            EXECUTOR.create_task(move || Source::new(i, a0));
            a1
        }

        fn map<A, B>(a: $($mod)::+::Pullable<A>, f: fn(A) -> B) -> $($mod)::+::Pullable<B>
        where
            A: Data,
            B: Data,
        {
            let (b0, b1) = $($mod)::+::channel(&EXECUTOR);
            EXECUTOR.create_task(move || Map::new(a, f, b0));
            b1
        }

        fn log<T>(a: $($mod)::+::Pullable<T>)
        where
            T: Data,
        {
            EXECUTOR.create_task(move || Log::new(a));
        }

        fn plus_one(x: i32) -> i32 {
            x + 1
        }

        #[test]
        fn main() {
            let s = source(0..100);
            let s = map(s, plus_one);
            let _ = log(s);
        }
    }
}

mod test1 {
    compile_test!(arc_runtime::channels::remote::concurrent);
}

mod test2 {
    compile_test!(arc_runtime::channels::remote::broadcast);
}

mod test3 {
    compile_test!(arc_runtime::channels::local::concurrent);
}

mod test4 {
    compile_test!(arc_runtime::channels::local::broadcast);
}
