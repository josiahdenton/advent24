use std::error::Error;

use anyhow::{anyhow, Result};
use clap::{arg, Command};

use days::{day1, day2};

pub mod days;

fn cli() -> Command {
    Command::new("aoc").args([
        arg!(-d --day <day> "day argument"),
        arg!(-p --part <part> "part argument"),
        arg!(<file> "input file"),
    ])
}

#[derive(PartialEq, Eq, Debug)]
pub enum Part {
    P1,
    P2,
}

struct RunOptions {
    day: u32,
    part: Part,
    input_file_path: String,
}

fn fetch_run_options() -> RunOptions {
    let matches = cli().get_matches();

    let day = match matches.get_one::<String>("day") {
        Some(day) => day.parse::<u32>().expect("bad day argument"),
        None => panic!("missing day argument"),
    };

    let part = match matches.get_one::<String>("part") {
        Some(part) => match part.as_str() {
            "p1" | "P1" => Part::P1,
            "p2" | "P2" => Part::P2,
            _ => panic!("invalid part argument usage"),
        },
        None => panic!("missing part argument"),
    };

    let input_file_path = match matches.get_one::<String>("file") {
        Some(path) => String::from(path),
        None => panic!("missing file path argument"),
    };

    RunOptions {
        day,
        part,
        input_file_path,
    }
}

pub fn run() -> Result<()> {
    let options = fetch_run_options();

    match options.day {
        1 => day1::process(options.part, options.input_file_path),
        2 => day2::process(options.part, options.input_file_path),
        _ => Err(anyhow!("invalid day")),
    }
}
