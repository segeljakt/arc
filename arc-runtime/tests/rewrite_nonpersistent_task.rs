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

#[rewrite(nonpersistent)]
mod map {
    fn task(i: Pullable<i32>, f: fn(i32) -> i32, #[output] o: Pushable<i32>) {
        loop {
            push!(o, f(pull!(i)));
        }
    }
}

#[rewrite(nonpersistent)]
mod log {
    fn task(i: Pullable<i32>) {
        loop {
            info!(log, "Logging {}", pull!(i));
        }
    }
}

#[test]
fn rewrite_impersistent_task() {
    EXECUTOR.init(KompactConfig::default().build().unwrap());
    log(map(source((0..100).into_iter().collect()), |x| x + 1));
    EXECUTOR.await_termination();
}
