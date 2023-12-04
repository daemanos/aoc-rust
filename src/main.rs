use aoc_client::{self, AocClient, AocResult, PuzzleDay, PuzzleYear};
use chrono::{Datelike, FixedOffset, TimeZone, Utc};
use clap::{Parser, Subcommand};
use std::cmp;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};

use aoc2023;

const DECEMBER: u32 = 12;
const FIRST_PUZZLE_DAY: PuzzleDay = 1;
const LAST_PUZZLE_DAY: PuzzleDay = 25;
const RELEASE_TIMEZONE_OFFSET: i32 = -5 * 3600;

const TOKEN_PATH: &str = ".token";

#[derive(Parser, Debug)]
#[command(infer_subcommands = true)]
struct Args {
    #[command(subcommand)]
    command: Command,

    /// Puzzle year [default: year of current or last AoC event]
    #[arg(short, long, global = true)]
    year: Option<PuzzleYear>,

    /// Puzzle day [default: current day]
    #[arg(short, long, global = true)]
    day: Option<PuzzleDay>,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(visible_alias = "r")]
    Run {
        /// Puzzle part
        part: i64,

        /// Puzzle input within input/dayNN [default: input]
        #[arg(short, long)]
        input: Option<String>,

        /// Output [default: stdout]
        #[arg(short, long)]
        output: Option<String>,
    },

    #[command(visible_alias = "s")]
    Submit {
        /// Puzzle part
        part: i64,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // date stuff copied from aoc-client because it doesn't expose it >:(
    let now = FixedOffset::east_opt(RELEASE_TIMEZONE_OFFSET).unwrap()
        .from_utc_datetime(&Utc::now().naive_utc());

    let year = args.year.unwrap_or_else(|| {
        if now.month() < DECEMBER {
            now.year() - 1
        } else {
            now.year()
        }
    });

    let day = args.day.unwrap_or_else(|| {
        if year == now.year() && now.month() == DECEMBER {
            cmp::min(LAST_PUZZLE_DAY, now.day())
        } else if year < now.year() {
            LAST_PUZZLE_DAY
        } else {
            FIRST_PUZZLE_DAY
        }
    });

    let client = AocClient::builder()
        .year(year)?
        .day(day)?
        .session_cookie_from_file(TOKEN_PATH)?
        .build()?;

    match args.command {
        Command::Run { part, input, output } => {
            let input = get_input(input, year, day, &client)?;
            let mut output = get_output(output)?;

            let answer = aoc2023::solve(&input, year, day, part.try_into()?);
            writeln!(output, "{answer}")?;
            Ok(())
        },
        Command::Submit { .. } => {
            todo!()
        },
    }
}

fn get_input(
    input: Option<String>,
    year: PuzzleYear,
    day: PuzzleDay,
    client: &AocClient,
) -> AocResult<String> {
    let input = input.unwrap_or("input".to_string());
    let input_dir = format!("input/{}/day{:02}", year, day);
    let input_path = format!("{input_dir}/{input}");

    fs::read_to_string(&input_path).or_else(|_| {
        let input = client.get_input()?;

        // try to create the input file and save the downloaded input to it
        // if initial creation fails, mkdir -p the directory and try again
        let write = File::create(&input_path)
            .or_else(|_| fs::create_dir_all(&input_dir)
                .and_then(|_| File::create(&input_path)))
            .and_then(|mut file| writeln!(file, "{}", input.trim()));

        if let Err(err) = write {
            eprintln!("warning: failed to save input: {err}");
        };

        Ok(input)
    })
}

fn get_output(output: Option<String>) -> Result<Box<dyn Write>, io::Error> {
    Ok(match output {
        None => Box::new(io::stdout()),
        Some(path) => Box::new(File::create(&path)?),
    })
}
