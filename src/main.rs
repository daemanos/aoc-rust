use std::fs::{self, File};
use std::io::{self, Write};
use std::error::Error;

use aoc_client::{AocClient, PuzzleDay, PuzzleYear, PuzzlePart};
use clap::Parser;

use aoc2023;

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

    aoc2023::solve(&input, args.day, part, &mut output)
}

