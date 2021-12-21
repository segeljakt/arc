# RUN: arc-lang %s | arc-mlir-rust-test %t - -rustinclude %s.rust-tests
# RUN: arc-lang %s | arc-mlir-rust-test %t-canon - -rustinclude %s.rust-tests -canonicalize

def plus_one(stream: Stream[i32]): Stream[i32] {
    [emit event+1; for event in stream; if event != 0]
}
