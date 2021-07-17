use crate::solver;
use std::str::FromStr;

/// A set of values that describes the conditions for when a password is valid.
#[derive(Debug)]
struct PasswordPolicy {
    range: (usize, usize),
    letter: char,
}

impl FromStr for PasswordPolicy {
    type Err = solver::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        let mut range_iter = iter
            .next()
            .ok_or_else(|| solver::Error::from_desc("password policy must have a range"))?
            .split('-')
            .map(|s| s.parse::<usize>());
        let mut next_num_in_range = || -> Result<usize, solver::Error> {
            let num = range_iter
                .next()
                .ok_or_else(|| {
                    solver::Error::from_desc("password policy range must have two numbers")
                })?
                .map_err(|e| {
                    solver::Error::new(e, "cannot parse number in password policy range as integer")
                })?;
            if num > 0 {
                Ok(num)
            } else {
                Err(solver::Error::from_desc(
                    "number in password policy range must be positive",
                ))
            }
        };

        let letter = iter
            .next()
            .ok_or_else(|| solver::Error::from_desc("password policy must have a letter"))?;
        if letter.len() != 1 {
            return Err(solver::Error::from_desc(
                "password policy letter must be a single character",
            ));
        }
        let letter = letter.chars().next().unwrap();

        Ok(PasswordPolicy {
            range: (next_num_in_range()?, next_num_in_range()?),
            letter,
        })
    }
}

/// Solves the puzzle for day 2 of 2020.
pub fn solve(input: &str) -> solver::Result {
    let mut solutions = Vec::new();

    let passwords = input
        .lines()
        .map(|ln| {
            let mut iter = ln.split(": ");
            let policy: PasswordPolicy = iter
                .next()
                .ok_or_else(|| solver::Error::from_desc("password policy cannot be empty"))?
                .parse::<PasswordPolicy>()?;
            let password = iter
                .next()
                .ok_or_else(|| solver::Error::from_desc("password cannot be empty"))?;
            Ok((policy, password))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Guarantee that the range of every password policy is properly oriented; i.e., the first
    // number is less than or equal to the second
    for (policy, _) in &passwords {
        if policy.range.0 > policy.range.1 {
            return Err(solver::Error::from_desc(
                "password policy range is not properly oriented",
            ));
        }
    }
    solutions.push(
        (passwords
            .iter()
            .filter(|(policy, password)| {
                let occurrences = password.matches(policy.letter).count();
                return policy.range.0 <= occurrences && occurrences <= policy.range.1;
            })
            .count())
        .to_string(),
    );

    // Guarantee that the numbers in the range of every password policy refer to valid letters in
    // the policy
    for (policy, password) in &passwords {
        if policy.range.0 > password.len() || policy.range.1 > password.len() {
            return Err(solver::Error::from_desc(format!("password policy expects letter at position {}, but password is only {} letters long", policy.range.0, password.len())));
        }
    }
    solutions.push(
        (passwords
            .iter()
            .filter(|(policy, password)| {
                let mut chars = password.chars();
                let first = chars.nth(policy.range.0 - 1).unwrap();
                let second = chars.nth(policy.range.1 - policy.range.0 - 1).unwrap();
                // Given a password policy containing two positions, a password is valid iff:
                //
                // (first position contains the desired letter) XOR (second position contains the
                // desired letter)
                (first == policy.letter) != (second == policy.letter)
            })
            .count())
        .to_string(),
    );

    Ok(solutions)
}
