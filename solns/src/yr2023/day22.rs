use std::ops::Index;

use crate::Soln;
use utils::prelude::*;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = ();

    fn part1(input: &str) -> Self::Answer {
        //let mut blocking = HashSet::new();
        //let mut supports = HashMap::new();

    }

    fn part2(_input: &str) -> Self::Answer {
        unsolved!()
    }
}

fn parse(input: &str) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = input.lines()
        .map(|line| line.into())
        .collect();

    bricks.sort_by_key(|brick| brick.origin().2);
    bricks
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Brick {
    Cube(Point3d),
    Rect(Point3d, Axis3d, usize),
}
use Brick::*;

impl Brick {
    fn on_axis(from: Point3d, axis: Axis3d, to: Point3d) -> Self {
        Self::Rect(from, axis, from.dist_on_axis(axis, to))
    }

    fn origin(self) -> Point3d {
        match self {
            Self::Cube(o) => o,
            Self::Rect(o, _, _) => o,
        }
    }

    fn map<F: Fn(Point3d) -> Point3d>(self, f: F) -> Self {
        match self {
            Self::Cube(o) => Self::Cube(f(o)),
            Self::Rect(o, a, s) => Self::Rect(f(o), a, s),
        }
    }

    fn at_height(self, height: usize) -> Self {
        self.map(|o| Point3d(o.0, o.1, height))
    }

    fn overlaps(self, other: Self) -> bool {
        match (self, other) {
            (Self::Cube(p), Self::Cube(q)) => p == q,
            (Self::Cube(p), Self::Rect(q, a, s))
                | (Self::Rect(q, a, s), Self::Cube(p)) =>
                p.on_axis(a, q) && p[a].clamp(q[a], q[a] + s) == p[a],
            (Self::Rect(p, a, s), Self::Rect(q, b, t)) => {
                if a == Z && b == Z {
                    p.on_axis(Z, q) &&
                        cmp::max(p[Z], q[Z]) <= cmp::min(p[Z] + s, q[Z] + t)
                } else if a == Z {
                    todo!()
                } else {
                    todo!()
                }
            }
        }
    }
}

impl From<&str> for Brick {
    fn from(s: &str) -> Self {
        let ends = s.split_once('~').unwrap();
        let from: Point3d = ends.0.into();
        let to: Point3d = ends.1.into();

        if from == to {
            Self::Cube(from)
        } else if from.on_axis(X, to) {
            Self::on_axis(from, X, to)
        } else if from.on_axis(Y, to) {
            Self::on_axis(from, Y, to)
        } else if from.on_axis(Z, to) {
            Self::on_axis(from, Z, to)
        } else {
            panic!()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point3d(usize, usize, usize);

impl Point3d {
    fn on_axis(self, axis: Axis3d, other: Self) -> bool {
        match axis {
            X => self[Y] == other[Y] && self[Z] == other[Z],
            Y => self[X] == other[X] && self[Z] == other[Z],
            Z => self[X] == other[X] && self[Y] == other[Y],
        }
    }

    fn dist_on_axis(self, axis: Axis3d, other: Self) -> usize {
        other[axis] - self[axis] + 1
    }
}

impl Index<Axis3d> for Point3d {
    type Output = usize;
    fn index(&self, index: Axis3d) -> &Self::Output {
        match index {
            X => &self.0,
            Y => &self.1,
            Z => &self.2,
        }
    }
}

impl From<&str> for Point3d {
    fn from(s: &str) -> Self {
        let parts: Vec<usize> = s.split(',')
            .map(|w| w.parse().unwrap())
            .collect();

        match parts.as_slice() {
            &[x, y, z] => Self(x, y, z),
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Axis3d {
    X,
    Y,
    Z,
}
use Axis3d::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn part1() {
        assert_eq!((), Puzzle::part1(""));
    }

    #[test]
    #[ignore]
    fn part2() {
        assert_eq!((), Puzzle::part2(""));
    }
}
