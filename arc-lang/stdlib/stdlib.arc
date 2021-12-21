@{intrinsic: "none"}
extern type unit;

@{intrinsic: "si8"}
extern type i8;

@{intrinsic: "si16"}
extern type i16;

# ------------------------------------------------------

@{intrinsic: "si32"}
extern type i32;

@{intrinsic: "add_si32"}
extern def +(i32, i32): i32;

@{intrinsic: "sub_si32"}
extern def -(i32, i32): i32;

@{intrinsic: "mul_si32"}
extern def *(i32, i32): i32;

@{intrinsic: "div_si32"}
extern def /(i32, i32): i32;

@{intrinsic: "pow_si32"}
extern def **(i32, i32): i32;

@{intrinsic: "rem_si32"}
extern def %(i32, i32): i32;

@{intrinsic: "geq_si32"}
extern def >=(i32, i32): bool;

@{intrinsic: "leq_si32"}
extern def <=(i32, i32): bool;

@{intrinsic: "gt_si32"}
extern def >(i32, i32): bool;

@{intrinsic: "lt_si32"}
extern def <(i32, i32): bool;

@{intrinsic: "bor_si32"}
extern def bor(i32, i32): bool;

@{intrinsic: "bxor_si32"}
extern def bxor(i32, i32): bool;

@{intrinsic: "band_si32"}
extern def band(i32, i32): bool;

@{intrinsic: "neg_si32"}
extern def neg(i32): i32;

extern def to_string(i32): String;

# ------------------------------------------------------

@{intrinsic: "si64"}
extern type i64;

@{intrinsic: "u64"}
extern type u8;

@{intrinsic: "u128"}
extern type u16;

@{intrinsic: "u64"}
extern type u32;

@{intrinsic: "u128"}
extern type u64;

@{intrinsic: "u64"}
extern type u128;

@{intrinsic: "f32"}
extern type f32;

@{intrinsic: "f64"}
extern type f64;

# ------------------------------------------------------

@{intrinsic: "i1"}
extern type bool;

@{intrinsic: "and_bool"}
extern def and(bool, bool): bool;

@{intrinsic: "or_bool"}
extern def or(bool, bool): bool;

@{intrinsic: "xor_bool"}
extern def xor(bool, bool): bool;

@{intrinsic: "not_bool"}
extern def not(bool): bool;

# ------------------------------------------------------

extern type char;

# ------------------------------------------------------

extern type String;

extern def concat(String, String): String;

# ------------------------------------------------------

extern type Option[T];
extern def is_some[T](Option[T]): bool;
extern def is_none[T](Option[T]): bool;
extern def unwrap[T](Option[T]): T;
extern def unwrap_or[T](Option[T], T): T;
extern def unwrap_or_else[T](Option[T], fun():T): T;

# ------------------------------------------------------

extern type Array[T];

extern def new_array[T](T): Array[T];
extern def push_array[T](Array[T], T): unit;
extern def pop_array[T](Array[T]): unit;
extern def select_array[T](Array[T], i32): T;
extern def len_array[T](Array[T]): i32;
extern def extend_array[T](Array[T], Array[T]): unit;

# ------------------------------------------------------

extern type Cell[T];

extern def new_cell[T](T): Cell[T];
extern def update_cell[T](Cell[T], T): unit;
extern def read_cell[T](Cell[T]): T;

# ------------------------------------------------------

extern type Iter[T];

extern def next[T](Iter[T], T): T;

# ------------------------------------------------------

extern type Range[T];

extern def new_range[T](T, T): Range[T];
extern def leq_range[T](Range[T], T): bool;
extern def geq_range[T](Range[T], T): bool;
extern def lt_range[T](Range[T], T): bool;
extern def gt_range[T](Range[T], T): bool;

# ------------------------------------------------------

extern type Stream[T];

extern def map[A,B](Stream[A], fun(A):B): Stream[B];
extern def key_by[K,V](Stream[V], fun(V):K): KStream[K,V];

# ------------------------------------------------------

extern type KStream[K,V];

extern def fold[K,V](KStream[K,V], fun(V,V):V): Stream[V];
