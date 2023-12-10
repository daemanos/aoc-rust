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
