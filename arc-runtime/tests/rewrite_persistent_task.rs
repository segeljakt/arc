use arc_runtime::prelude::*;

static EXECUTOR: Executor = Executor::new();

#[rewrite(nonpersistent)]
mod source {
    fn task(i: Vec<i32>, #[output] o: Pushable<i32>) {
        for x in i {
            push!(o, x);
        }
    }
}

#[rewrite(persistent)]
mod map {
    fn task(a: Pullable<i32>, f: fn(i32) -> i32, #[output] b: Pushable<i32>) {}

    struct State0 {
        a: Pullable<i32>,
        f: fn(i32) -> i32,
        b: Pushable<i32>,
    }

    struct State1 {
        a: Pullable<i32>,
        f: fn(i32) -> i32,
        b: Pushable<i32>,
        pull: BoxFuture<'static, Control<i32>>,
    }

    struct State2 {
        a: Pullable<i32>,
        f: fn(i32) -> i32,
        b: Pushable<i32>,
        push: BoxFuture<'static, Control<()>>,
    }

    enum State {
        State0(State0),
        State1(State1),
        State2(State2),
    }

    fn transition0(State0 { a, b, f }: State0, _cx: &mut PollContext) -> (Poll<()>, State) {
        let pull = a.pull().boxed();
        let pull = unsafe { std::mem::transmute(pull) };
        (Poll::Pending, State1 { a, b, f, pull }.into())
    }

    fn transition1(
        State1 { a, b, f, mut pull }: State1,
        cx: &mut PollContext,
    ) -> (Poll<()>, State) {
        if let Poll::Ready(Control::Continue(x)) = pull.as_mut().poll(cx) {
            let push = b.push(x).boxed();
            let push = unsafe { std::mem::transmute(push) };
            (Poll::Pending, State2 { a, b, f, push }.into())
        } else {
            (Poll::Pending, State1 { a, b, f, pull }.into())
        }
    }

    fn transition2(
        State2 { a, b, f, mut push }: State2,
        cx: &mut PollContext,
    ) -> (Poll<()>, State) {
        if let Poll::Ready(Control::Continue(())) = push.as_mut().poll(cx) {
            (Poll::Pending, State0 { a, b, f }.into())
        } else {
            (Poll::Pending, State2 { a, b, f, push }.into())
        }
    }
}

#[rewrite(nonpersistent)]
mod log {
    fn task(i: Pullable<i32>) {
        loop {
            println!("Logging {}", pull!(i));
        }
    }
}

#[test]
fn rewrite_impersistent_task() {
    EXECUTOR.init(KompactConfig::default().build().unwrap());
    log(map(source((0..100).into_iter().collect()), |x| x + 1));
    EXECUTOR.await_termination();
}
