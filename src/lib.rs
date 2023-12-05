pub mod utils;

use std::fmt::Debug;

use aoc_client::{PuzzleYear, PuzzleDay, PuzzlePart};

pub trait Soln {
    type Answer: Debug;

    fn part1(input: &str) -> Self::Answer;
    fn part2(input: &str) -> Self::Answer;

    fn solve(input: &str, part: PuzzlePart) -> String {
        let answer = match part {
            PuzzlePart::PartOne => Self::part1(input),
            PuzzlePart::PartTwo => Self::part2(input),
        };

        format!("{:?}", answer).into()
    }
}

pub fn solve(input: &str, year: PuzzleYear, day: PuzzleDay, part: PuzzlePart) -> String {
    let doit = match year {
        2023 => yr2023::solve,
         _ => todo!(),
    };

    doit(input, day, part)
}

mod yr2023;
