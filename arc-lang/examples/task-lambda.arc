# RUN: arc-lang %s | arc-mlir-rust-test %t - -rustinclude %s.rust-tests
# RUN: arc-lang %s | arc-mlir-rust-test %t-canon - -rustinclude %s.rust-tests -canonicalize

def test(s: Stream[i32]): Stream[i32] {
    s | task: loop { on event => emit event } 
}
