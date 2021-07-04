mod year2020;

use chrono::prelude::*;
use dotenv::dotenv;
use log::{error, info};
use std::{collections::HashMap, default::Default, env, fmt, path::PathBuf, str::FromStr, time::Instant};
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

fn all_puzzles() -> HashMap<(&'static Year, &'static Day), fn(&str) -> String> {
    let mut puzzles: HashMap<(&'static Year, &'static Day), fn(&str) -> String> = HashMap::new();
    puzzles.insert((&Year(2020), &Day(1)), year2020::day01::solve);
    puzzles
}

fn main() {
    dotenv().ok();

    env_logger::init();

    let opt = Opt::from_args();

    let session = env::var("SESSION").ok();

    let puzzles = all_puzzles();
    if let Some(&solve) = puzzles.get(&(&opt.year, &opt.day)) {
        let before_solving = Instant::now();
        let solution = solve("");
        info!("Solution found in {:.3} ms", before_solving.elapsed().as_secs_f64() * 1000.);
        info!("Solution: {}", solution);
    } else {
        error!("No solver found for day {} of {}", opt.day, opt.year);
    }
}
