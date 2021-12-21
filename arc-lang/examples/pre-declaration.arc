# RUN: arc-lang %s | arc-mlir-rust-test %t - -rustinclude %s.rust-tests
# RUN: arc-lang %s | arc-mlir-rust-test %t-canon - -rustinclude %s.rust-tests -canonicalize

def add(a:i32, b:i32): i32;

def add(a, b) {
    a + b
}

# Update def

