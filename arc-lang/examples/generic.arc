# RUN: arc-lang %s | arc-mlir-rust-test %t - -rustinclude %s.rust-tests
# RUN: arc-lang %s | arc-mlir-rust-test %t-canon - -rustinclude %s.rust-tests -canonicalize

def identity[A](x: A, y: A): A {
    identity(x, y)
}

def main() {
    val x: i32 = 5;
    identity::[i32](x, x);
}

task Identity[A](): A -> A {
    loop {
        on event => emit event;
    }
}
