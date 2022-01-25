# RUN: arc-lang %s | arc-mlir-rust-test %t - -rustinclude %s.rust-tests
# RUN: arc-lang %s | arc-mlir-rust-test %t-canon - -rustinclude %s.rust-tests -canonicalize

# ANCHOR: example
enum Shape[T] {
    Rectangle(T, T),
    Circle(T),
}

def area(shape) = match shape {
    Shape::Rectangle(width, height) => width * height,
    Shape::Circle(radius) => 3.14 * radius ** 2
}

def test() {
    val a0 = area(Shape::Rectangle(5.0, 3.0));
    val a1 = area(Shape::Circle(3.0));
}
# ANCHOR_END: example
