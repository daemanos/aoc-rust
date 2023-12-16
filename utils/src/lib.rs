#![allow(dead_code)]
#![feature(pattern)]
#![feature(const_option)]

pub mod algo;
pub mod client;
pub mod convert;
pub mod geom;
pub mod grid;
pub mod intervals;
pub mod math;
pub mod types;

pub mod prelude {
    // Modules
    pub use super::*;

    pub use geom::*;
    pub use grid::{Grid, Vec2D, Dim, IdxPoint};
    pub use intervals::*;

    // Enum variants
    pub use Direction::*;
    pub use Axis::*;

    // Macros
    pub use utils_derive::*;
}


pub trait IterUtils<'a, T: 'a> {
    fn combos(&'a self) -> Combos<T>;
    fn pairs(&'a self) -> impl Iterator<Item = (&'a T, &'a T)>;
}

// TODO blanket implementation for Into<&[T]>

impl<'a, T: 'a> IterUtils<'a, T> for &'a [T] {
    fn combos(&'a self) -> Combos<T> {
        Combos { xs: self, n: self.len(), i: 0, j: 0 }
    }

    fn pairs(&'a self) -> impl Iterator<Item = (&'a T, &'a T)> {
        self.windows(2)
            .map(|w| match &w {
                &[x1, x2] => (x1, x2),
                _ => panic!(),
            })
    }
}

impl<'a, T: 'a> IterUtils<'a, T> for Vec<T> {
    fn combos(&'a self) -> Combos<T> {
        let xs = self.as_slice();
        Combos { xs, n: self.len(), i: 0, j: 0 }
    }

    fn pairs(&'a self) -> impl Iterator<Item = (&'a T, &'a T)> {
        let xs = self.as_slice();
        xs.windows(2)
            .map(|w| match &w {
                &[x1, x2] => (x1, x2),
                _ => panic!(),
            })
    }
}

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

#[macro_export]
macro_rules! unsolved {
    () => {
        panic!("not solved yet");
    };
}
