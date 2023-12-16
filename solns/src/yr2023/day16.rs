use std::collections::HashSet;

use crate::Soln;
use utils::prelude::*;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = usize;

    fn part1(input: &str) -> Self::Answer {
        let grid: Vec2D<Cell> = input.parse().unwrap();
        num_energized(&grid, Beam { pos: Point(1, 1), dir: Direction::E })
    }

    fn part2(input: &str) -> Self::Answer {
        let grid: Vec2D<Cell> = input.parse().unwrap();
        let Dim(h, w) = grid.dim();

        let top = (1..w)
            .map(|col| Beam {pos: Point(1, col), dir: Direction::S});
        let btm = (1..w)
            .map(|col| Beam {pos: Point(h, col), dir: Direction::N});
        let left = (1..h)
            .map(|row| Beam {pos: Point(row, 1), dir: Direction::E});
        let right = (1..h)
            .map(|row| Beam {pos: Point(row, w), dir: Direction::W});

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
                    (Direction::S, Right) => Direction::W,
                    (Direction::W, Right) => Direction::S,
                    (Direction::N, Right) => Direction::E,
                    (Direction::E, Right) => Direction::N,
                    (Direction::S, Left) => Direction::E,
                    (Direction::E, Left) => Direction::S,
                    (Direction::N, Left) => Direction::W,
                    (Direction::W, Left) => Direction::N,
                    _ => panic!(),
                };
                Sng(self.cont(dir))
            }
            Cell::Split(axis) => {
                match (self.dir, axis) {
                    (Direction::S | Direction::N, Vert) =>
                        Sng(self.cont(self.dir)),
                    (Direction::E | Direction::W, Vert) =>
                        Split(self.cont(Direction::N), self.cont(Direction::S)),
                    (Direction::E | Direction::W, Horiz) =>
                        Sng(self.cont(self.dir)),
                    (Direction::S | Direction::N, Horiz) =>
                        Split(self.cont(Direction::E), self.cont(Direction::W)),
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

    #[test]
    #[ignore]
    fn part1() {
        //assert_eq!((), Puzzle::part1(""));
    }

    #[test]
    #[ignore]
    fn part2() {
        //assert_eq!((), Puzzle::part2(""));
    }
}
