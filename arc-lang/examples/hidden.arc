def foo(bar) {
    bar(1, 2)
}

def test() {
    val x = foo(_ + _);
}
