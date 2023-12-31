//! Fundamental 2D geometric primitives.

use std::ops::{Add, Sub};
use std::cmp::Ordering;
use std::fmt;

use num_traits::{NumCast, Float, PrimInt, Signed};

pub static NEIGHBORS: [Direction; 8] = [
    Direction::NW, Direction::N, Direction::NE,
    Direction::W,                Direction::E,
    Direction::SW, Direction::S, Direction::SE,
];

pub static CARDINAL_DIRS: [Direction; 4] = [
    Direction::N, Direction::E, Direction::S, Direction::W
];

/// A general-purpose lattice point. Order: (row, col)
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point<T>(pub T, pub T);

impl<T> Copy for Point<T>
where T: Copy {}

impl<T> Clone for Point<T>
where T: Clone {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<T> fmt::Display for Point<T>
where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

/// Iterator over neighbors of a point
pub struct Neighbors<T> {
    origin: Point<T>,
    neighbors: &'static [Direction],
    idx: usize,
}

impl<T> Neighbors<T> {
    fn new(origin: Point<T>, neighbors: &'static [Direction]) -> Self {
        Self { origin, neighbors, idx: 0 }
    }
}

impl<T> Iterator for Neighbors<T>
where T: PrimInt {
    type Item = Point<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.neighbors.get(self.idx).and_then(|&neighbor| {
            let result = Some(self.origin + neighbor);
            self.idx += 1;

            result
        })
    }
}

impl<T> Point<T>
where T: PrimInt {
    /// Get all neighbors of a point (including diagonally)
    pub fn neighbors(self) -> Neighbors<T> {
        Neighbors::new(self, &NEIGHBORS)
    }

    /// Get only the orthogonal neighbors of a point
    pub fn ortho_neighbors(self) -> Neighbors<T> {
        Neighbors::new(self, &CARDINAL_DIRS)
    }

    pub fn ortho_to(self, other: Self) -> bool {
        let one = T::one();
        match (self.0.cmp(&other.0), self.1.cmp(&other.1)) {
            (Ordering::Less,    Ordering::Equal)   => self.0 == other.0 - one,
            (Ordering::Greater, Ordering::Equal)   => self.0 == other.0 + one,
            (Ordering::Equal,   Ordering::Less)    => self.1 == other.1 - one,
            (Ordering::Equal,   Ordering::Greater) => self.1 == other.1 + one,
            _ => false,
        }
    }

    pub fn colinear(&self, other: &Self) -> bool {
        (self.0 == other.0) ^ (self.1 == other.1)
    }
}

impl<T> Point<T>
where T: PrimInt + Signed {
    pub fn abs(&self) -> Self {
        Self(self.0.abs(), self.1.abs())
    }

    pub fn dist<F: Float + NumCast>(p1: Self, p2: Self) -> Option<F> {
        let Self(dx, dy) = (p1 - p2).abs();

        let dx = F::from(dx)?;
        let dy = F::from(dy)?;
        Some((dx.powi(2) + dy.powi(2)).sqrt())
    }

    pub fn signum(&self) -> Self {
        Self(self.0.signum(), self.1.signum())
    }

    pub fn direction(&self) -> Option<Direction> {
        let one = T::one();
        let zero = T::zero();
        let signum = self.signum();

        if signum.0 == zero {
            if signum.1 == one {
                Some(Direction::E)
            } else if signum.1 == -one {
                Some(Direction::W)
            } else {
                None
            }
        } else if signum.1 == zero {
            if signum.0 == one {
                Some(Direction::S)
            } else if signum.0 == -one {
                Some(Direction::N)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<T> Add for Point<T>
where T: PrimInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> Sub for Point<T>
where T: PrimInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Direction {
    NW, N, NE,
     W,    E,
    SW, S, SE,
}

impl Direction {
    pub fn cardinal(&self) -> bool {
        match *self {
            Self::N | Self::W | Self::E | Self::S => true,
            _ => false,
        }
    }

    pub fn unit<T: PrimInt + Signed>(&self) -> Point<T> {
        let zero = T::zero();
        let one = T::one();
        let (y, x) = match *self {
            Self::NW => (-one, -one),
            Self::W  => (zero, -one),
            Self::NE => (-one,  one),
            Self::N  => (-one, zero),
            Self::S  => ( one, zero),
            Self::SW => ( one, -one),
            Self::E  => (zero,  one),
            Self::SE => ( one,  one),
        };

        Point(y, x)
    }

    pub fn rot90(&self, mag: Rot) -> Self {
        match (*self, mag) {
            (Self::N, Rot::Pos) => Self::W,
            (Self::N, Rot::Neg) => Self::E,
            (Self::E, Rot::Pos) => Self::N,
            (Self::E, Rot::Neg) => Self::S,
            (Self::S, Rot::Pos) => Self::E,
            (Self::S, Rot::Neg) => Self::W,
            (Self::W, Rot::Pos) => Self::S,
            (Self::W, Rot::Neg) => Self::N,
            (Self::NW, Rot::Pos) => Self::SW,
            (Self::NW, Rot::Neg) => Self::NE,
            (Self::NE, Rot::Pos) => Self::NW,
            (Self::NE, Rot::Neg) => Self::SE,
            (Self::SE, Rot::Pos) => Self::NE,
            (Self::SE, Rot::Neg) => Self::SW,
            (Self::SW, Rot::Pos) => Self::SE,
            (Self::SW, Rot::Neg) => Self::NW,
        }
    }

    pub fn opp(&self) -> Self {
        match self {
            Self::NW => Self::SE,
            Self::N  => Self::S,
            Self::NE => Self::SW,
            Self::W  => Self::E,
            Self::SE => Self::NW,
            Self::S  => Self::N,
            Self::SW => Self::NE,
            Self::E  => Self::W,
        }
    }
}

impl<T> Add<Direction> for Point<T>
where T: PrimInt {
    type Output = Self;
    fn add(self, rhs: Direction) -> Self::Output {
        let one = T::one();
        match rhs {
            Direction::NW => Self(self.0 - one, self.1 - one),
            Direction::W  => Self(self.0,       self.1 - one),
            Direction::NE => Self(self.0 - one, self.1 + one),
            Direction::N  => Self(self.0 - one, self.1      ),
            Direction::S  => Self(self.0 + one, self.1      ),
            Direction::SW => Self(self.0 + one, self.1 - one),
            Direction::E  => Self(self.0,       self.1 + one),
            Direction::SE => Self(self.0 + one, self.1 + one),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Rot {
    Neg,
    Pos,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Axis {
    Vert,
    Horiz,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn abs() {
        assert_eq!(Point(3_i32, 4_i32), Point(-3_i32, 4_i32).abs());
        assert_eq!(Point(3_i32, 4_i32), Point(3_i32, -4_i32).abs());
        assert_eq!(Point(3_i32, 4_i32), Point(-3_i32, -4_i32).abs());
    }

    #[test]
    fn dist() {
        let p1 = Point(0_i32, 0_i32);
        let p2 = Point(3_i32, 4_i32);
        let dist: f32 = Point::dist(p1, p2).unwrap();
        assert_eq!(5.0, dist);
    }

    #[test]
    fn add() {
        assert_eq!(Point(4_u32, 5_u32), Point(1_u32, 2_u32) + Point(3_u32, 3_u32));
        assert_eq!(Point(0_i32, 0_i32), Point(-4_i32, 1_i32) + Point(4_i32, -1_i32));
    }

    #[test]
    fn sub() {
        assert_eq!(Point(1_u32, 0_u32), Point(5_u32, 3_u32) - Point(4_u32, 3_u32));
        assert_eq!(Point(-1_i32, 1_i32), Point(0_i32, 0_i32) - Point(1_i32, -1_i32));
    }

    #[test]
    fn neighbors() {
        let origin = Point(0_i32, 0_i32);

        let expected = HashSet::from([
            Point(-1, -1), Point(0, -1), Point(1, -1),
            Point(-1,  0),               Point(1,  0),
            Point(-1,  1), Point(0,  1), Point(1,  1),
        ]);
        let actual: HashSet<_> = origin.neighbors().collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn ortho_neighbors() {
        let origin = Point(0_i32, 0_i32);

        let expected = HashSet::from([
            Point(0, -1), Point(1, 0), Point(0, 1), Point(-1, 0)
        ]);
        let actual: HashSet<_> = origin.ortho_neighbors().collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn ortho_to() {
        let origin = Point(0_i32, 0_i32);

        assert!(origin.ortho_to(Point( 0,  1)));
        assert!(origin.ortho_to(Point( 1,  0)));
        assert!(origin.ortho_to(Point( 0, -1)));
        assert!(origin.ortho_to(Point(-1,  0)));

        assert!(!origin.ortho_to(Point( 1,  1)));
        assert!(!origin.ortho_to(Point( 1, -1)));
        assert!(!origin.ortho_to(Point(-1, -1)));
        assert!(!origin.ortho_to(Point(-1,  1)));
    }
}
