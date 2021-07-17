mod solver;
mod year2020;

use anyhow::Context;
use chrono::prelude::*;
use dotenv::dotenv;
use log::{error, info, warn};
use reqwest::{blocking::Client, header};
use std::{
    collections::HashMap,
    default::Default,
    env,
    fmt::{self, Display, Formatter},
    fs::{self, File},
    io::Write,
    path::PathBuf,
    str::FromStr,
    time::Instant,
};
use structopt::StructOpt;

/// Newtype for a year. Allows any command-line arguments that take a year to use the current year
/// as a default value.
#[derive(Debug, Eq, Hash, PartialEq)]
struct Year(i32);

impl Default for Year {
    fn default() -> Self {
        Self(Local::now().year())
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Year {
    type Err = <i32 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

/// Newtype for a day. Allows any command-line arguments that take a day to use the current day as a
/// default value.
#[derive(Debug, Eq, Hash, PartialEq)]
struct Day(u32);

impl Default for Day {
    fn default() -> Self {
        Self(Local::now().day())
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Day {
    type Err = <u32 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
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

/// A function that solves a puzzle.
type Solver = fn(&str) -> solver::Result;

/// Returns a map that maps a tuple containing a year and day to the function which solves the
/// puzzle for that year and day.
fn all_puzzles() -> HashMap<(i32, u32), Solver> {
    let mut puzzles: HashMap<(i32, u32), Solver> = HashMap::new();
    puzzles.insert((2020, 1), year2020::day01::solve);
    puzzles.insert((2020, 2), year2020::day02::solve);
    puzzles.insert((2020, 3), year2020::day03::solve);
    puzzles
}

/// Retrieves the input for a puzzle from https://adventofcode.com, which requires the environment
/// variable `SESSION` to be set (see `.env.template`). If the environment variable `CACHE_DIR` is
/// set, then the puzzle input will be read from/written to the cache as appropriate.
fn retrieve_input(year: &Year, day: &Day) -> anyhow::Result<String> {
    let result = env::var("CACHE_DIR");
    let cache_path = match result {
        Ok(cache_dir) => {
            let mut path = PathBuf::new();
            path.push(cache_dir);
            path.push(format!("{}", year.0));
            path.push(format!("{}.txt", day.0));
            Some(path)
        }
        Err(_) => None,
    };

    // Attempt to read puzzle input from the cache, but do nothing if an error occurs
    if let Some(ref cache_path) = cache_path {
        let result = fs::read_to_string(cache_path);
        if let Ok(input) = result {
            info!("Found puzzle input in cache");
            return Ok(input);
        }
    }

    let session = env::var("SESSION")
        .context("no session found, go to https://adventofcode.com to obtain a session")?;

    let client = Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year.0, day.0);
    info!("Retrieving puzzle input from https://adventofcode.com...");
    let input = client
        .get(&url)
        .header(header::COOKIE, format!("session={}", session))
        .send()
        .with_context(|| format!("could not make request to {}", url))?
        .text()
        .with_context(|| format!("could not parse response from {}", url))?;
    info!("Successfully retrieved puzzle input");

    // Attempt to write puzzle input to the cache
    match cache_path {
        Some(cache_path) => {
            let dir = cache_path.parent().unwrap();
            fs::create_dir_all(dir).with_context(|| {
                format!("could not create directory '{}' in cache", dir.display())
            })?;

            let mut cache_file = File::create(&cache_path).with_context(|| {
                format!("could not create file '{}' in cache", cache_path.display())
            })?;

            cache_file.write_all(input.as_bytes()).with_context(|| {
                format!(
                    "could not write to file '{}' in cache",
                    cache_path.display()
                )
            })?;
        }
        None => {
            warn!("could not store puzzle input in cache; ensure that CACHE_DIR is set in .env");
        }
    };

    Ok(input)
}

/// Launches the application.
fn main() -> anyhow::Result<()> {
    let result = dotenv();
    if let Err(e) = result {
        warn!("{}", anyhow::Error::new(e));
    }

    env_logger::init();

    let opt = Opt::from_args();

    let puzzles = all_puzzles();
    if let Some(&solve) = puzzles.get(&(opt.year.0, opt.day.0)) {
        let input = if let Some(path) = opt.input {
            fs::read_to_string(&path)
                .with_context(|| format!("cannot read from input file '{}'", path.display()))?
        } else {
            retrieve_input(&opt.year, &opt.day)?
        };

        let before_solving = Instant::now();

        info!("Solving puzzle for day {} of {}...", opt.day, opt.year);
        let solutions = solve(&input).context("cannot solve puzzle")?;

        if !solutions.is_empty() {
            info!(
                "{} solution{} found in {:.3} ms",
                solutions.len(),
                if solutions.len() == 1 { "" } else { "s" },
                before_solving.elapsed().as_secs_f64() * 1000.
            );
            for (idx, solution) in solutions.iter().enumerate() {
                println!("Part {}: {}", idx + 1, solution);
            }
        } else {
            warn!("No solutions found")
        }
    } else {
        error!("No solver found for day {} of {}", opt.day, opt.year);
    }
    Ok(())
}
