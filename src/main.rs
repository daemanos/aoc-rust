use std::fmt::Debug;
use std::fs::{self, File};
use std::io::{self, Write};
use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    day: u8,
    part: u8,

    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let solve = match args.day {
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
        day => return Err(format!("illegal day: {day}").into()),
    };

    let input = args.input.unwrap_or("input".to_string());
    let input = fs::read_to_string(format!("input/day{:02}/{}", args.day, input))?;

    let mut output: Box<dyn Write> = match args.output {
        None => Box::new(io::stdout()),
        Some(path) => Box::new(File::create(&path)?),
    };

    solve(&input, args.part, &mut output)?;
    Ok(())
}

trait Soln {
    type Answer: Debug;

    fn part1(input: &str) -> Self::Answer;
    fn part2(input: &str) -> Self::Answer;

    fn solve(input: &str, part: u8, output: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let answer = match part {
            1 => Self::part1(input),
            2 => Self::part2(input),
            _ => return Err(format!("illegal part: {part}").into()),
        };

        writeln!(output, "{:?}", answer)?;
        Ok(())
    }
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
