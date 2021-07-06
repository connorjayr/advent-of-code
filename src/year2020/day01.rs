use crate::solver;
use std::collections::HashSet;

pub fn solve(input: &str) -> solver::Result {
    let mut solutions = Vec::new();

    let expenses = input
        .lines()
        .map(|s| s.parse::<i32>())
        .collect::<Result<HashSet<_>, _>>()?;

    for expense in expenses.iter() {
        if expenses.contains(&(2020 - expense)) {
            solutions.push((expense * (2020 - expense)).to_string());
            break;
        }
    }

    'outer: for expense1 in expenses.iter() {
        for expense2 in expenses.iter() {
            if expenses.contains(&(2020 - expense1 - expense2)) {
                solutions.push((expense1 * expense2 * (2020 - expense1 - expense2)).to_string());
                break 'outer;
            }
        }
    }

    Ok(solutions)
}
