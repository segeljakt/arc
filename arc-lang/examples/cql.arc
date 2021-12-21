# XFAIL: arc-lang %s | arc-mlir-rust-test %t - -rustinclude %s.rust-tests
# XFAIL: arc-lang %s | arc-mlir-rust-test %t-canon - -rustinclude %s.rust-tests -canonicalize

def test0(s) {
  from x in s {
    where x.k != 1
    group x.k
      reduce
        sum of x.v,
        count
  }
}

def test1(s: Stream[#{k:i32,v:i32}]) {
  from x in s {
    where x.k != 1
    group k = x.k
      reduce
        sum = sum of x.v,
        count = count
  }
}
