# RUN: arc-lang %s | arc-mlir-rust-test %t - -rustinclude %s.rust-tests
# RUN: arc-lang %s | arc-mlir-rust-test %t-canon - -rustinclude %s.rust-tests -canonicalize

def fib(x: i32): i32 {
    if x > 1 {
        fib(x-1) + fib(x-2)
    } else {
        x
    }
}
