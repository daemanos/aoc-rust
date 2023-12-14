use std::collections::HashMap;

use crate::Soln;
use utils::{Charty, Point, Direction, Grid, Vec2D, Dim};

// note: input is square

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = usize;

    fn part1(input: &str) -> Self::Answer {
        let mut platform = Platform(input.parse().unwrap());
        platform.roll(Direction::N);
        platform.load()
    }

    fn part2(input: &str) -> Self::Answer {
        let mut platform = Platform(input.parse().unwrap());

        let mut seen: HashMap<Platform, usize> = HashMap::new();
        let n = 1_000_000_000;
        for i in 0..n {
            platform.roll(Direction::N);
            platform.roll(Direction::W);
            platform.roll(Direction::S);
            platform.roll(Direction::E);

            match seen.get(&platform) {
                Some(&s) => {
                    let period = i - s + 1;
                    return if period > 0 {
                        let end = (n - s) % period;
                        seen.iter()
                            .find(|(_, &i)| i == s + end)
                            .map(|(platform, _)| platform.load())
                            .unwrap()
                    } else {
                        platform.load()
                    };
                },
                None => {
                    seen.insert(platform.clone(), i + 1);
                },
            };
        }

        platform.load()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Platform(Vec2D<Cell>);

impl Platform {
    fn load(&self) -> usize {
        let Dim(s, _) = self.0.dim();

        let mut sum = 0;
        for row in 1..=s {
            for col in 1..=s {
                if self.0[Point(row, col)] == RoundRock {
                    sum += s - row + 1;
                }
            }
        }

        sum
    }

    fn roll(&mut self, dir: Direction) {
        let Dim(s, _) = self.0.dim();

        let mkpoint = |a, b| match dir {
            Direction::N | Direction::S => Point(b, a),
            Direction::E | Direction::W => Point(a, b),
            _ => panic!(),
        };

        let init_empty = match dir {
            Direction::N | Direction::W => 1,
            Direction::S | Direction::E => s,
            _ => panic!(),
        };

        let succ = |x| match dir {
            Direction::N | Direction::W => x + 1,
            Direction::S | Direction::E => x - 1,
            _ => panic!(),
        };

        let bs: Vec<usize> = match dir {
            Direction::N | Direction::W => (1..=s).collect(),
            Direction::S | Direction::E => (1..=s).rev().collect(),
            _ => panic!(),
        };

        for a in 1..=s {
            let mut next_empty = init_empty;
            for &b in bs.iter() {
                let p = mkpoint(a, b);
                match self.0[p] {
                    RoundRock => {
                        let q = mkpoint(a, next_empty);
                        if p != q {
                            self.0[q] = RoundRock;
                            self.0[p] = Empty;
                        }
                        next_empty = succ(next_empty);
                    },
                    CubeRock => next_empty = succ(b),
                    _ => (),
                };
            }
        }
    }

    #[allow(unused)]
    fn draw(&self) {
        let Dim(s, _) = self.0.dim();
        let mut buf = String::new();
        for row in 1..=s {
            for col in 1..=s {
                buf.push(self.0[Point(row, col)].into());
            }
            buf.push('\n');
        }
        println!("{buf}");
    }
}

#[derive(Debug, Hash, Charty)]
#[repr(u8)]
enum Cell {
    RoundRock = b'O',
    CubeRock = b'#',
    Empty = b'.',
}

use Cell::*;

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part1() {
        assert_eq!(136, Puzzle::part1(&INPUT));
    }

    #[test]
    fn part2() {
        assert_eq!(64, Puzzle::part2(&INPUT));
    }
}
