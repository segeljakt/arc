# RUN: arc-lang %s | arc-mlir-rust-test %t - -rustinclude %s.rust-tests
# RUN: arc-lang %s | arc-mlir-rust-test %t-canon - -rustinclude %s.rust-tests -canonicalize

def test() {
# ANCHOR: example
val a = 1;
val b = true;

# Arithmetic
- a;
a + a;
a - a;
a * a;
a / a;
a ** a;
a % a;

# Equality
a == a;
a != a;

# Logical
b and b;
b or b;
b band b;
b bor b;
b bxor b;
not b;

# Containers
a in [a, a, a];
a not in [];
# ANCHOR_END: example
}
