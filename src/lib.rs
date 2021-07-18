pub mod solver;
pub mod year2020;

use anyhow::Context;
use chrono::prelude::*;
use log::{info, warn};
use reqwest::{blocking::Client, header};
use std::{
    env,
    fmt::{self, Display, Formatter},
    fs::{self, File},
    io::Write,
    path::PathBuf,
    str::FromStr,
};

/// Newtype for a year. Allows any command-line arguments that take a year to use the current year
/// as a default value.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Year(i32);

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
pub struct Day(u32);

impl Default for Day {
    fn default() -> Self {
        Self(Local::now().day())
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Day {
    type Err = <u32 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

/// Retrieves the input for a puzzle from https://adventofcode.com, which requires the environment
/// variable `SESSION` to be set (see `.env.template`). If the environment variable `CACHE_DIR` is
/// set, then the puzzle input will be read from/written to the cache as appropriate.
pub fn retrieve_input(year: &Year, day: &Day) -> anyhow::Result<String> {
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
