use std::fs::{self, File};
use std::io::{self, Write};
use std::error::Error;

use aoc_client::{self, AocClient, AocResult, PuzzleDay, PuzzleYear};
use clap::{Parser, Subcommand};

use aoc2023;

const TOKEN_PATH: &str = ".token";

#[derive(Parser, Debug)]
#[command(infer_subcommands = true)]
struct Args {
    #[command(subcommand)]
    command: Command,

    /// Puzzle year [default: year of current or last AoC event]
    #[arg(short, long, global = true)]
    year: Option<PuzzleYear>,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(visible_alias = "r")]
    Run {
        /// Puzzle day
        day: PuzzleDay,

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
        /// Puzzle day
        day: PuzzleDay,

        /// Puzzle part
        part: i64,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let client = build_client(&args)?;

    match args.command {
        Command::Run { day, part, input, output } => {
            let input = get_input(input, 2023, day, &client)?;
            let mut output = get_output(output)?;

            let answer = aoc2023::solve(&input, 2023, day, part.try_into()?);
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
    let input_path = format!("input/{}/day{:02}/{}", year, day, input);

    fs::read_to_string(&input_path).or_else(|_| {
        let input = client.get_input()?;

        let write = File::create(&input_path)
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

fn build_client(args: &Args) -> AocResult<AocClient> {
    let mut builder = AocClient::builder();

    match args.year {
        Some(year) => builder.year(year)?,
        None => builder.latest_event_year()?,
    };

    match args.command {
        Command::Run { day, .. } | Command::Submit { day, .. } => builder.day(day)?,
    };

    builder
        .session_cookie_from_file(TOKEN_PATH)?
        .build()
}
