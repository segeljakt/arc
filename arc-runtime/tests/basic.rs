#![allow(unused_mut)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![feature(arbitrary_self_types)]
use arc_runtime::channels::local::concurrent::*;
use arc_runtime::prelude::*;

// Note: This may only be used by the main thread.
static EXECUTOR: Executor = Executor::new();

#[derive(ComponentDefinition)]
struct DoThing {
    ctx: ComponentContext<Self>,
    a: Pullable<i32>,
    b: Pullable<i32>,
    c: Pushable<i32>,
}

impl Actor for DoThing {
    type Message = TaskMessage;

    fn receive_local(&mut self, msg: Self::Message) -> Handled {
        Handled::Ok
    }

    fn receive_network(&mut self, msg: kompact::prelude::NetMessage) -> Handled {
        Handled::Ok
    }
}

impl DoThing {
    fn new(a: Pullable<i32>, b: Pullable<i32>, c: Pushable<i32>) -> Self {
        Self {
            ctx: ComponentContext::uninitialised(),
            a,
            b,
            c,
        }
    }

    async fn run(mut self: ComponentDefinitionAccess<Self>) -> Control<()> {
        loop {
            let x = self.a.pull().await?;
            let y = self.b.pull().await?;
            self.c.push(x + y).await?;
        }
        Control::Finished
    }
}

fn do_thing(a: Pullable<i32>, b: Pullable<i32>) -> Pullable<i32> {
    let (c0, c1): (Pushable<i32>, Pullable<i32>) = channel(&EXECUTOR);
    EXECUTOR.create_task(move || DoThing::new(a, b, c0));
    c1
}

impl ComponentLifecycle for DoThing {
    fn on_start(&mut self) -> Handled {
        self.spawn_local(move |async_self| async move {
            async_self.run().await;
            Handled::DieNow
        });
        futures::task::waker_ref(&self.ctx.typed_component());
        Handled::Ok
    }
}

fn read_stream() -> Pullable<i32> {
    todo!()
}

fn main() {
    EXECUTOR.init(KompactConfig::default().build().unwrap());
    let a: Pullable<i32> = read_stream();
    let b: Pullable<i32> = read_stream();
    let c = do_thing(a, b);
}
