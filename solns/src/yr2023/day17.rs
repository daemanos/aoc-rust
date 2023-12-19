use std::collections::{BinaryHeap, HashSet};

use crate::Soln;
use utils::prelude::*;

const MOVES: [Move; 3] = [Move::Left, Move::Right, Move::Straight];

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = u32;

    fn part1(input: &str) -> Self::Answer {
        let grid: Vec2D<Cell> = input.parse().unwrap();
        let start = State::new(E, Point(1, 1), None, Some(3));

        min_path(&grid, start)
    }

    fn part2(input: &str) -> Self::Answer {
        let grid: Vec2D<Cell> = input.parse().unwrap();
        let start = State::new(E, Point(1, 1), Some(4), Some(10));

        min_path(&grid, start)
    }
}

fn min_path(grid: &Vec2D<Cell>, start: State) -> u32 {
    let Dim(h, w) = grid.dim();
    let goal = Point(h, w);

    let mut seen = HashSet::new();
    let mut front = BinaryHeap::from([start]);
    //let mut paths = vec![];
    let mut min = u32::MAX;

    while let Some(state) = front.pop() {
        let node = (state.pos, state.dir, state.straight);
        if !seen.contains(&node) {
            seen.insert(node);

            for mv in MOVES {
                if let Some(next) = state.succ(&grid, mv) {

                    if next.is_goal(goal) {
                        //paths.push(next.score);
                        min = cmp::min(next.score, min);
                    } else {
                        front.push(next);
                    }
                }
            }
        }
    }

    //for (score, path) in paths {
    //    if score == min {
    //        let mut buf = String::new();
    //        for row in 1..=h {
    //            for col in 1..=w {
    //                let idx = Point(row, col);
    //                buf.push(match path.get(&idx) {
    //                    Some(&N) => '^',
    //                    Some(&S) => 'v',
    //                    Some(&W) => '<',
    //                    Some(&E) => '>',
    //                    _ => char::from_digit(grid[idx].0, 10).unwrap(),
    //                });
    //            }
    //            buf.push('\n');
    //        }
    //        println!("\n{buf}");
    //    }
    //}

    min
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Cell(u32);
impl From<char> for Cell {
    fn from(ch: char) -> Self {
        Self(ch.to_digit(10).unwrap())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct State {
    dir: Direction,
    pos: IdxPoint,
    score: u32,
    straight: u8,
    min_straight: Option<u8>,
    max_straight: Option<u8>,
    //hist: HashMap<IdxPoint, Direction>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Note: direction reversed for min-heap behavior
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl State {
    fn new(
        dir: Direction,
        pos: IdxPoint,
        min_straight: Option<u8>,
        max_straight: Option<u8>,
    ) -> Self {
        Self { dir, pos, score: 0, straight: 1, min_straight, max_straight }
    }

    fn is_goal(&self, pos: IdxPoint) -> bool {
        self.pos == pos && self.min_straight.unwrap_or(0) <= self.straight
    }

    fn to(&self, score: u32, dir: Direction, pos: IdxPoint, mv: Move) -> Self {
        let score = self.score + score;
        let straight = if mv == Move::Straight {
            self.straight + 1
        } else {
            1
        };

        Self {
            dir,
            pos,
            score,
            straight,
            min_straight: self.min_straight,
            max_straight: self.max_straight,
        }
    }

    fn succ(&self, grid: &Vec2D<Cell>, mv: Move) -> Option<Self> {
        let dir = mv.in_dir(self.dir);
        let pos = self.pos + dir;
        grid.get(pos).and_then(|&score| {
            let next = self.to(score.0, dir, pos, mv);
            match (self.min_straight, self.max_straight, mv) {
                (_, Some(max), Move::Straight)
                    if self.straight < max => Some(next),
                (_, None, Move::Straight) => Some(next),

                (Some(min), _, Move::Left | Move::Right)
                    if self.straight >= min => Some(next),
                (None, _, Move::Left | Move::Right) => Some(next),

                _ => None,
            }
        })
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Move {
    Left,
    Right,
    Straight,
}

impl Move {
    fn in_dir(&self, dir: Direction) -> Direction {
        match *self {
            Self::Straight => dir,
            Self::Left => dir.rot90(Rot::Pos),
            Self::Right => dir.rot90(Rot::Neg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str =
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part1() {
        assert_eq!(102, Puzzle::part1(&INPUT));
    }

    #[test]
    fn part2() {
        assert_eq!(94, Puzzle::part2(&INPUT));
        assert_eq!(71, Puzzle::part2(
                "111111111111
                 999999999991
                 999999999991
                 999999999991
                 999999999991"));

        assert_eq!(5, Puzzle::part2(
                "11111111111
                 99990999991
                 99990999991
                 99990999991
                 99990000001"));
    }
}
