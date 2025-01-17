// RUN: arc-mlir %s -split-input-file -verify-diagnostics

module @toplevel {
  func @bad() -> !arc.enum<a : i32, b : f32> {
    %a = arith.constant 4 : i32
    // expected-error@+2 {{'arc.make_enum' op : variant 'c' does not exist in '!arc.enum<a : i32, b : f32>'}}
    // expected-note@+1 {{see current operation:}}
    %r = arc.make_enum (%a : i32) as "c" : !arc.enum<a : i32, b : f32>
    return %r : !arc.enum<a : i32, b : f32>
  }
}

// -----

module @toplevel {
  func @bad() -> !arc.enum<a : i32, b : f32> {
    %a = arith.constant 3.14 : f32
    // expected-error@+2 {{'arc.make_enum' op : variant 'a' does not have a matching type, expected 'f32' but found 'i32'}}
    // expected-note@+1 {{see current operation:}}
    %r = arc.make_enum (%a : f32) as "a" : !arc.enum<a : i32, b : f32>
    return %r : !arc.enum<a : i32, b : f32>
  }
}

// -----

module @toplevel {
  func @access0(%e : !arc.enum<a : i32, b : f32>) -> i32 {
    // expected-error@+2 {{'arc.enum_access' op : variant 'b' does not have a matching type, expected 'i32' but found 'f32'}}
    // expected-note@+1 {{see current operation:}}
    %r = arc.enum_access "b" in (%e : !arc.enum<a : i32, b : f32>) : i32
    return %r : i32
  }
}

// -----

module @toplevel {
  func @access0(%e : !arc.enum<a : i32, b : f32>) -> i32 {
    // expected-error@+2 {{'arc.enum_access' op : variant 'c' does not exist in '!arc.enum<a : i32, b : f32>'}}
    // expected-note@+1 {{see current operation:}}
    %r = arc.enum_access "c" in (%e : !arc.enum<a : i32, b : f32>) : i32
    return %r : i32
  }
}

// -----

module @toplevel {
  func @check2(%e : !arc.enum<a : i32, b : f32>) -> i1 {
    // expected-error@+2 {{'arc.enum_check' op : variant 'c' does not exist in '!arc.enum<a : i32, b : f32>'}}
    // expected-note@+1 {{see current operation:}}
    %r = arc.enum_check (%e : !arc.enum<a : i32, b : f32>) is "c" : i1
    return %r : i1
  }
}

// -----

module @toplevel {
  func @bad() -> !arc.enum<bad : i32, b : f32> {
    %a = arith.constant 4 : i32
    // expected-error@+2 {{'arc.make_enum' op : variant 'bad' does not have a matching type, expected 'none' but found 'i32'}}
    // expected-note@+1 {{see current operation}}
    %r = arc.make_enum () as "bad" : !arc.enum<bad : i32, b : f32>
    return %r : !arc.enum<bad : i32, b : f32>
  }
}

// -----

module @toplevel {
  func @bad() -> !arc.enum<a : i32, b : f32> {
    %b = arith.constant 3.14 : f32
    // expected-error@+2 {{'arc.make_enum' op : only a single value expected}}
    // expected-note@+1 {{see current operation}}
    %r = arc.make_enum (%b, %b : f32, f32) as "b" : !arc.enum<a : i32, b : f32>
    return %r : !arc.enum<a : i32, b : f32>
  }
}

// -----
module @toplevel {
   func @access2(%e : !arc.enum<a : si32, b : f32, no_value : none>) -> none {
    // expected-error@+2 {{accessing a 'none'-typed variant does not make sense}}
    // expected-note@+1 {{see current operation}}
    %r = arc.enum_access "no_value" in (%e : !arc.enum<a : si32, b : f32, no_value : none>) : none
    return %r : none
  }
}
