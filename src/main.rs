mod year2020;

use anyhow::Context;
use chrono::prelude::*;
use dotenv::dotenv;
use log::{error, info};
use reqwest::{blocking::Client, header};
use std::{
    collections::HashMap, default::Default, env, fmt, fs, path::PathBuf, str::FromStr,
    time::Instant,
};
use structopt::StructOpt;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Year(i32);

impl Default for Year {
    fn default() -> Year {
        Year(Local::now().year())
    }
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Year {
    type Err = <i32 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Year, Self::Err> {
        Ok(Year(s.parse()?))
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Day(u32);

impl Default for Day {
    fn default() -> Day {
        Day(Local::now().day())
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Day {
    type Err = <u32 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Day, Self::Err> {
        Ok(Day(s.parse()?))
    }
}

/// Command-line options provided when running this application.
#[derive(StructOpt, Debug)]
#[structopt(name = "advent_of_code")]
struct Opt {
    /// The year in which the puzzle to solve was released
    #[structopt(short, long, default_value)]
    year: Year,

    /// The day on which the puzzle to solve was released
    #[structopt(short, long, default_value)]
    day: Day,

    #[structopt(short, long)]
    input: Option<PathBuf>,
}

fn all_puzzles() -> HashMap<(i32, u32), fn(&str) -> String> {
    let mut puzzles: HashMap<(i32, u32), fn(&str) -> String> = HashMap::new();
    puzzles.insert((2020, 1), year2020::day01::solve);
    puzzles
}

fn retrieve_input(year: &Year, day: &Day) -> anyhow::Result<String> {
    let session = env::var("SESSION")
        .context("No session found, go to https://adventofcode.com to obtain a session")?;

    let client = Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year.0, day.0);
    client
        .get(&url)
        .header(header::COOKIE, format!("session={}", session))
        .send()
        .with_context(|| format!("Could not make request to {}", url))?
        .text()
        .with_context(|| format!("Could not parse response from {}", url))
}

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    env_logger::init();

    let opt = Opt::from_args();

    let puzzles = all_puzzles();
    if let Some(&solve) = puzzles.get(&(opt.year.0, opt.day.0)) {
        let input = if let Some(path) = opt.input {
            fs::read_to_string(&path)
                .with_context(|| format!("Could not read from input file '{}'", path.display()))?
        } else {
            retrieve_input(&opt.year, &opt.day)?
        };

        let before_solving = Instant::now();

        let solution = solve(&input);

        info!(
            "Solution found in {:.3} ms",
            before_solving.elapsed().as_secs_f64() * 1000.
        );
        info!("Solution: {}", solution);
    } else {
        error!("No solver found for day {} of {}", opt.day, opt.year);
    }
    Ok(())
}
