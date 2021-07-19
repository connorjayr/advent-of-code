use advent_of_code::{self, solver, Day, Year};
use anyhow::Context;
use dotenv::dotenv;
use log::{error, info, warn};
use std::{
    fs::{self},
    path::PathBuf,
    time::Instant,
};
use structopt::StructOpt;

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

/// Launches the application.
fn main() -> anyhow::Result<()> {
    let result = dotenv();
    if let Err(e) = result {
        warn!("{}", anyhow::Error::new(e));
    }

    env_logger::init();

    let opt = Opt::from_args();

    let puzzles = solver::all_puzzles();
    if let Some(&solve) = puzzles.get(&(opt.year.0, opt.day.0)) {
        let input = if let Some(path) = opt.input {
            fs::read_to_string(&path)
                .with_context(|| format!("cannot read from input file '{}'", path.display()))?
        } else {
            advent_of_code::retrieve_input(&opt.year, &opt.day)?
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
