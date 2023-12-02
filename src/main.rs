use std::fmt::Debug;
use std::fs::{self, File};
use std::io::{self, Write};
use std::error::Error;

use aoc_client::{AocClient, PuzzleDay, PuzzleYear, PuzzlePart};
use clap::Parser;

const TOKEN_PATH: &str = ".token";

const YEAR: PuzzleYear = 2023;

#[derive(Parser, Debug)]
struct Args {
    day: PuzzleDay,
    part: i64,

    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let client = AocClient::builder()
        .session_cookie_from_file(TOKEN_PATH)?
        .year(YEAR)?
        .day(args.day)?
        .build()?;

    let part: PuzzlePart = args.part.try_into()?;

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
         _ => panic!(),
    };

    let input = args.input.unwrap_or("input".to_string());
    let input_path = format!("input/day{:02}/{}", args.day, input);
    let input = match fs::read_to_string(&input_path) {
        Ok(str) => str,
        Err(_) => {
            let input = client.get_input()?;

            let write = File::create(&input_path)
                .and_then(|mut file| writeln!(file, "{}", input.trim()));
            if let Err(err) = write {
                eprintln!("warning: failed to save input: {err}");
            };

            input
        }
    };

    let mut output: Box<dyn Write> = match args.output {
        None => Box::new(io::stdout()),
        Some(path) => Box::new(File::create(&path)?),
    };

    solve(&input, part, &mut output)?;
    Ok(())
}

trait Soln {
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
