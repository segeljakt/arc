use arc_runtime::prelude::*;

declare_functions!(f);

#[rewrite]
fn f(x: i32) -> i32 {
    x + 1
}

#[rewrite(nonpersistent)]
mod source {
    fn task(i: Vec<i32>, #[output] o: Pushable<i32>) {
        for x in i.into_iter().cloned() {
            push!(o, x);
        }
    }
}

#[rewrite(nonpersistent)]
mod map {
    fn task(i: Pullable<i32>, f: function!((i32) -> i32), #[output] o: Pushable<i32>) {
        loop {
            let x = pull!(i);
            let y = call_indirect!(f(x));
            push!(o, y);
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

use arc_runtime::channels::local::task_parallel::Pullable;

#[rewrite(main)]
#[test]
fn rewrite_impersistent_task() {
    let v: Vec<i32> = vector![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let s: Pullable<i32> = call!(source(v));
    let f: function!((i32) -> i32) = function!(f);
    let s: Pullable<i32> = call!(map(s, f));
    call!(log(s));
}
