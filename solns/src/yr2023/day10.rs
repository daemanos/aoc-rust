use std::cmp;
use std::collections::{VecDeque, HashSet};

use crate::Soln;
use utils::prelude::*;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = u32;

    fn part1(input: &str) -> Self::Answer {
        let (grid, start) = parse_input(input);

        let mut steps = 0;
        let mut stack = VecDeque::new();
        let mut seen = HashSet::new();
        for &dir in &geom::CARDINAL_DIRS {
            let pos = start + dir;
            if let Some(cell) = grid.get(pos) {
                if cell.can_connect(dir) {
                    stack.push_front((1, pos));
                    seen.insert(pos);
                }
            }
        }

        while let Some((s, pos)) = stack.pop_back() {
            steps = cmp::max(steps, s);
            if let Some(Pipe(dir1, dir2)) = grid.get(pos) {
                let pos1 = pos + *dir1;
                if !seen.contains(&pos1) {
                    stack.push_front((s + 1, pos1));
                    seen.insert(pos1);
                }

                let pos2 = pos + *dir2;
                if !seen.contains(&pos2) {
                    stack.push_front((s + 1, pos2));
                    seen.insert(pos2);
                }
            }
        }

        steps
    }

    fn part2(input: &str) -> Self::Answer {
        let (grid, start) = parse_input(input);
        let Dim(h, w) = grid.dim();

        let mut pos = start;

        let mut dir = None;
        for &d in &geom::CARDINAL_DIRS {
            let pos = start + d;
            if let Some(cell) = grid.get(pos) {
                if cell.can_connect(d) {
                    dir = Some(d);
                    break;
                }
            }
        }
        let mut dir = dir.unwrap();

        let mut line = HashSet::from([start]);
        loop {
            if let Some(Pipe(d1, d2)) = grid.get(pos + dir) {
                pos = pos + dir;
                dir = if *d1 == dir.opp() {
                    *d2
                } else if *d2 == dir.opp() {
                    *d1
                } else {
                    panic!()
                };
            }

            if line.contains(&pos) {
                break;
            } else {
                line.insert(pos);
            }
        }

        let start_crossing = grid.get(start + Direction::S)
            .map(|cell| match cell {
                Pipe(d1, d2) => *d1 == Direction::N || *d2 == Direction::N,
                _ => false,
            }).unwrap_or(false);

        let mut enclosed = 0;
        for row in 0..h {
            let mut crossings = 0;
            for col in 0..w {
                let pos = Point(row, col);
                match grid.get(pos) {
                    Some(Start) => if start_crossing {
                        crossings += 1;
                    },
                    Some(cell) => if line.contains(&pos) {
                        if cell.is_crossing() {
                            crossings += 1;
                        };
                    } else {
                        if crossings % 2 == 1 {
                            enclosed += 1;
                        }
                    },
                    _ => (),
                };
            }
        }

        enclosed
    }
}

fn parse_input(input: &str) -> (Vec2D<Cell>, IdxPoint) {
    let grid: Vec2D<Cell> = input.parse().unwrap();
    let Dim(h, w) = grid.dim();

    let mut start = Point(0, 0);
    'row: for row in 1..=h {
        for col in 1..=w {
            let p = Point(row, col);
            if Start == grid[p] {
                start = p;
                break 'row;
            }
        }
    }

    (grid, start)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Pipe(Direction, Direction),
    Ground,
    Start,
}

use Cell::*;

impl Cell {
    fn can_connect(&self, from: Direction) -> bool {
        if let Self::Pipe(dir1, dir2) = self {
            from == dir1.opp() || from == dir2.opp()
        } else {
            false
        }
    }

    fn is_crossing(&self) -> bool {
        match self {
            Pipe(Direction::N, Direction::S) => true,
            Pipe(Direction::S, Direction::W) => true,
            Pipe(Direction::S, Direction::E) => true,
            _ => false,
        }
    }

    #[allow(unused)]
    fn to_char(self) -> char {
        match self {
            Pipe(Direction::N, Direction::S) => '║',
            Pipe(Direction::E, Direction::W) => '═',
            Pipe(Direction::N, Direction::E) => '╚',
            Pipe(Direction::N, Direction::W) => '╝',
            Pipe(Direction::S, Direction::W) => '╗',
            Pipe(Direction::S, Direction::E) => '╔',
            Ground => '.',
            Start => 'S',
            _ => panic!(),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = ();
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '|' => Ok(Self::Pipe(Direction::N, Direction::S)),
            '-' => Ok(Self::Pipe(Direction::E, Direction::W)),
            'L' => Ok(Self::Pipe(Direction::N, Direction::E)),
            'J' => Ok(Self::Pipe(Direction::N, Direction::W)),
            '7' => Ok(Self::Pipe(Direction::S, Direction::W)),
            'F' => Ok(Self::Pipe(Direction::S, Direction::E)),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str =
        "..F7.
         .FJ|.
         SJ.L7
         |F--J
         LJ...";

    #[test]
    fn part1() {
        assert_eq!(8, Puzzle::part1(&INPUT));
    }

    #[test]
    fn part2() {
        assert_eq!(4, Puzzle::part2(
                "..........
                 .S------7.
                 .|F----7|.
                 .||....||.
                 .||....||.
                 .|L-7F-J|.
                 .|..||..|.
                 .L--JL--J.
                 .........."));

        assert_eq!(8, Puzzle::part2(
                ".F----7F7F7F7F-7....
                 .|F--7||||||||FJ....
                 .||.FJ||||||||L7....
                 FJL7L7LJLJ||LJ.L-7..
                 L--J.L7...LJS7F-7L7.
                 ....F-J..F7FJ|L7L7L7
                 ....L7.F7||L7|.L7L7|
                 .....|FJLJ|FJ|F7|.LJ
                 ....FJL-7.||.||||...
                 ....L---J.LJ.LJLJ..."));

        assert_eq!(10, Puzzle::part2(
                "FF7FSF7F7F7F7F7F---7
                 L|LJ||||||||||||F--J
                 FL-7LJLJ||||||LJL-77
                 F--JF--7||LJLJ7F7FJ-
                 L---JF-JLJ.||-FJLJJ7
                 |F|F-JF---7F7-L7L|7|
                 |FFJF7L7F-JF7|JL---7
                 7-L-JL7||F7|L7F-7F7|
                 L.L7LFJ|||||FJL7||LJ
                 L7JLJL-JLJLJL--JLJ.L"));
    }
}
