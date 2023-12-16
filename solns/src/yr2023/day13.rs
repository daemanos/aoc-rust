use std::collections::HashSet;

use crate::Soln;
use utils::prelude::*;

type PointSet = HashSet<Point<usize>>;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = usize;

    fn part1(input: &str) -> Self::Answer {
        input.split("\n\n")
            .map(|pat| solve_pat(pat, Symmetry::is_valid))
            .sum()
    }

    fn part2(input: &str) -> Self::Answer {
        input.split("\n\n")
            .map(|pat| solve_pat(pat, Symmetry::can_fix))
            .sum()
    }
}

fn solve_pat<P>(input: &str, pred: P) -> usize
where P: Fn(&Symmetry) -> bool
{
    let mut points = HashSet::new();

    let mut dim = Dim(0, 0);
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                points.insert(Point(row, col));
            }
        }

        dim.0 += 1;
        dim.1 = line.trim().len();
    }

    let cols = (0..dim.1-1).map(|col| (col, Axis::Vert));
    let rows = (0..dim.0-1).map(|row| (row, Axis::Horiz));

    cols.chain(rows)
        .map(|(pos, axis)| Symmetry::new(&points, dim, pos, axis))
        .find_map(|symm| if pred(&symm) { Some(symm.value()) } else { None })
        .unwrap_or(0)
}

struct Symmetry {
    axis: Axis,
    pos: usize,
    class: SymmetryClass,
}

impl Symmetry {
    fn new(points: &PointSet, dim: Dim, pos: usize, axis: Axis) -> Self {
        let mut class = SymmetryClass::Valid;

        let dim = match axis {
            Axis::Vert => (dim.0, dim.1),
            Axis::Horiz => (dim.1, dim.0),
        };

        let mkpoint = |a, b| match axis {
            Axis::Vert => Point(a, b),
            Axis::Horiz => Point(b, a),
        };

        for offset in 0.. {
            if pos >= offset && pos + offset + 1 < dim.1 {
                let poss = (pos - offset, pos + offset + 1);
                for off2 in 0..dim.0 {
                    let p1 = mkpoint(off2, poss.0);
                    let p2 = mkpoint(off2, poss.1);
                    if points.contains(&p1) ^ points.contains(&p2) {
                        class = match class.next() {
                            Some(c) => c,
                            _ => break,
                        };
                    }
                }
            } else {
                break;
            }
        }

        Self { axis, pos, class }
    }

    fn is_valid(&self) -> bool {
        self.class == SymmetryClass::Valid
    }

    fn can_fix(&self) -> bool {
        self.class == SymmetryClass::Smudged
    }

    fn value(&self) -> usize {
        match self.axis {
            Axis::Vert => self.pos + 1,
            Axis::Horiz => 100 * (self.pos + 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum SymmetryClass {
    Valid,
    Smudged,
    Invalid,
}

impl SymmetryClass {
    fn next(&self) -> Option<Self> {
        match self {
            Self::Valid => Some(Self::Smudged),
            Self::Smudged => Some(Self::Invalid),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part1() {
        assert_eq!(405, Puzzle::part1(&INPUT));
    }

    #[test]
    fn part2() {
        assert_eq!(400, Puzzle::part2(&INPUT));
    }
}
