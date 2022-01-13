#![allow(clippy::type_complexity)]

use kompact::prelude::*;

use std::sync::Arc;

impl<I: DataReqs> Stream<I> {
    /// Apply a transformation to a stream to produce a new stream.
    /// ```ignore
    /// stream
    ///     .iterate(|s: Stream<i32>| {
    ///        let s1 = Filter(|x| x < 100) (s.clone());
    ///        let s2 = Map(x| x + 1) (s1);
    ///        let s3 = Filter(|x| x >= 100) (s);
    ///        (s1, s3)
    ///    })
    /// ```
    pub fn iterate<O: DataReqs>(
        self,
        f: fn(Stream<I>) -> (Stream<I>, Stream<O>),
    ) -> Stream<O> {
        todo!()
    }
}

pub struct Scope<const A: usize>([i32; A]);
