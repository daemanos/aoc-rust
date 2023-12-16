use std::collections::HashSet;

use crate::Soln;
use utils::prelude::*;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = usize;

    fn part1(input: &str) -> Self::Answer {
        let grid: Vec2D<Cell> = input.parse().unwrap();
        num_energized(&grid, Beam { pos: Point(1, 1), dir: E })
    }

    fn part2(input: &str) -> Self::Answer {
        let grid: Vec2D<Cell> = input.parse().unwrap();
        let Dim(h, w) = grid.dim();

        let top = (1..w).map(|col| Beam {pos: Point(1, col), dir: S});
        let btm = (1..w).map(|col| Beam {pos: Point(h, col), dir: N});
        let left = (1..h).map(|row| Beam {pos: Point(row, 1), dir: E});
        let right = (1..h).map(|row| Beam {pos: Point(row, w), dir: W});

        top.chain(btm).chain(left).chain(right)
            .map(|start| num_energized(&grid, start))
            .max()
            .unwrap()
    }
}

fn num_energized(grid: &Vec2D<Cell>, start: Beam) -> usize {
    let mut energized = HashSet::new();
    let mut seen = HashSet::new();
    let mut beams = vec![start];

    while let Some(beam) = beams.pop() {
        if !seen.contains(&beam) && grid.in_bounds(beam.pos) {
            seen.insert(beam);
            energized.insert(beam.pos);

            match beam.traverse(grid[beam.pos]) {
                Sng(beam) => beams.push(beam),
                Split(beam1, beam2) => {
                    beams.push(beam1);
                    beams.push(beam2);
                }
            }
        }
    }

    energized.len()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    pos: IdxPoint,
    dir: Direction,
}

enum BeamOut {
    Sng(Beam),
    Split(Beam, Beam),
}
use BeamOut::*;

impl Beam {
    fn traverse(&self, cell: Cell) -> BeamOut {
        match cell {
            Cell::Empty => Sng(self.cont(self.dir)),
            Cell::Refl(refl) => {
                let dir = match (self.dir, refl) {
                    (S, Right) => W,
                    (W, Right) => S,
                    (N, Right) => E,
                    (E, Right) => N,
                    (S, Left) => E,
                    (E, Left) => S,
                    (N, Left) => W,
                    (W, Left) => N,
                    _ => panic!(),
                };
                Sng(self.cont(dir))
            }
            Cell::Split(axis) => {
                match (self.dir, axis) {
                    (S | N, Vert) => Sng(self.cont(self.dir)),
                    (E | W, Vert) => Split(self.cont(N), self.cont(S)),
                    (E | W, Horiz) => Sng(self.cont(self.dir)),
                    (S | N, Horiz) => Split(self.cont(E), self.cont(W)),
                    _ => panic!(),
                }
            }
        }
    }

    fn cont(&self, dir: Direction) -> Self {
        Self { pos: self.pos + dir, dir }
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Refl(Reflection),
    Split(Axis),
}

#[derive(Debug, Clone, Copy)]
enum Reflection {
    Right,
    Left,
}
use Reflection::*;

impl From<char> for Cell {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Self::Empty,
            '/' => Self::Refl(Right),
            '\\' => Self::Refl(Left),
            '|' => Self::Split(Vert),
            '-' => Self::Split(Horiz),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str =
        ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn part1() {
        assert_eq!(46, Puzzle::part1(&INPUT));
    }

    #[test]
    fn part2() {
        assert_eq!(51, Puzzle::part2(&INPUT));
    }
}
