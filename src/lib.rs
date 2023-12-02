use std::fmt::Debug;
use std::io::Write;
use std::error::Error;

use aoc_client::{PuzzleDay, PuzzlePart};

pub trait Soln {
    type Answer: Debug;

    fn part1(input: &str) -> Self::Answer;
    fn part2(input: &str) -> Self::Answer;

    fn solve(input: &str, part: PuzzlePart, output: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let answer = match part {
            PuzzlePart::PartOne => Self::part1(input),
            PuzzlePart::PartTwo => Self::part2(input),
        };

        writeln!(output, "{:?}", answer)?;
        Ok(())
    }
}

pub fn solve(input: &str, day: PuzzleDay, part: PuzzlePart, output: &mut dyn Write) -> Result<(), Box<dyn Error>> {
    let doit = match day {
         1 => day01::Soln::solve,
         2 => day02::Soln::solve,
         3 => day03::Soln::solve,
         4 => day04::Soln::solve,
         5 => day05::Soln::solve,
         6 => day06::Soln::solve,
         7 => day07::Soln::solve,
         8 => day08::Soln::solve,
         9 => day09::Soln::solve,
        10 => day10::Soln::solve,
        11 => day11::Soln::solve,
        12 => day12::Soln::solve,
        13 => day13::Soln::solve,
        14 => day14::Soln::solve,
        15 => day15::Soln::solve,
        16 => day16::Soln::solve,
        17 => day17::Soln::solve,
        18 => day18::Soln::solve,
        19 => day19::Soln::solve,
        20 => day20::Soln::solve,
        21 => day21::Soln::solve,
        22 => day22::Soln::solve,
        23 => day23::Soln::solve,
        24 => day24::Soln::solve,
        25 => day25::Soln::solve,
         _ => panic!(),
    };

    doit(input, part, output)
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
