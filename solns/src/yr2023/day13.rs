use std::collections::HashSet;

use crate::Soln;
use utils::Point;

type PointSet = HashSet<Point<usize>>;
type Dim = (usize, usize);

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

    let mut dim = (0, 0);
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
    invalid: usize,
}

impl Symmetry {
    fn new(points: &PointSet, dim: Dim, pos: usize, axis: Axis) -> Self {
        let mut invalid = 0;

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
                        invalid += 1;
                    }
                }
            } else {
                break;
            }
        }

        Self { axis, pos, invalid }
    }

    fn is_valid(&self) -> bool {
        self.invalid == 0
    }

    fn can_fix(&self) -> bool {
        self.invalid == 1
    }

    fn value(&self) -> usize {
        match self.axis {
            Axis::Vert => self.pos + 1,
            Axis::Horiz => 100 * (self.pos + 1),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Axis {
    Vert,
    Horiz,
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
