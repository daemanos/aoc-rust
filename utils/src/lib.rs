#![allow(dead_code)]
#![feature(pattern)]

pub mod algo;
pub mod convert;
pub mod grid;
pub mod intervals;
pub mod math;
pub mod point;
pub mod types;

pub use convert::PeekFrom;
pub use grid::{Grid, Vec2D};
pub use intervals::{Interval, IntervalTree};
pub use point::{Point, Direction};

pub use utils_derive::FromWords;


pub struct Combos<'a, T> {
    xs: &'a [T],
    i: usize,
    j: usize,
    n: usize,
}

impl<'a, T> Iterator for Combos<'a, T> {
    type Item = (&'a T, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.n {
            None
        } else {
            let res = (&self.xs[self.i], &self.xs[self.j]);

            self.j += 1;
            if self.j == self.n {
                self.i += 1;
                self.j = self.i;
            }

            Some(res)
        }
    }
}

pub fn combos<T>(xs: &[T]) -> Combos<T> {
    let n = xs.len();
    Combos { xs, n, i: 0, j: 0 }
}
