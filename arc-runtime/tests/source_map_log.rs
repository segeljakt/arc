#![feature(arbitrary_self_types)]
#![allow(unused_mut)]

macro_rules! compile_test {
    {$($mod:tt)::+} => {
        use arc_runtime::prelude::*;
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

        fn plus_one(x: i32) -> i32 {
            x + 1
        }

        #[test]
        fn main() {
            let system = KompactConfig::default().build().unwrap();
            {
                let (pushable0, pullable0) = $($mod)::+::channel(&system);
                let (pushable1, pullable1) = $($mod)::+::channel(&system);
                system.create_task(move || Source::new(0..100, pushable0));
                system.create_task(move || Map::new(pullable0, plus_one, pushable1));
                system.create_task(move || Log::new(pullable1));
            }
            system.await_termination();
        }
    }
}

mod test1 {
    compile_test!(arc_runtime::channels::remote::concurrent);
}
// mod test2 {
//     compile_test!(arc_runtime::channels::remote::broadcast);
// }
// mod test3 {
//     compile_test!(arc_runtime::channels::local::concurrent);
// }
// mod test4 {
//     compile_test!(arc_runtime::channels::local::broadcast);
// }
