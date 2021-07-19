use crate::solver;
use std::vec::Vec;

/// A slope, which represents a step on a two-dimensional map. The first value corresponds to
/// horizontal movement whereas the second value corresponds to vertical movement.
type Slope = (usize, usize);

/// A two-dimensional map of the area, where `#` represents a tree and `.` represents an open
/// square.
struct Map {
    height: usize,
    width: usize,
    map: Vec<String>,
}

impl Map {
    /// Constructs a new `Map` from a two-dimensional vector.
    ///
    /// # Examples
    ///
    /// The following example creates a map from a 3×3 area:
    /// ```
    /// let result = Map::new(vec![vec!['.'; 3]; 3]);
    /// assert!(result.is_ok());
    /// ```
    ///
    /// The following example returns an error since the map is not rectangular:
    /// ```should_panic
    /// let result = Map::new(vec![vec!['.'; 3], vec!['.'; 2]]);
    /// assert!(result.is_err());
    /// ```
    pub fn new(map: Vec<String>) -> Result<Self, solver::Error> {
        let width = map.get(0).unwrap_or(&String::from("")).len();
        if map.iter().all(|row| row.len() == width) {
            Ok(Map {
                height: map.len(),
                width,
                map,
            })
        } else {
            Err(solver::Error::from_desc("map must be rectangular"))
        }
    }

    /// Counts the number of trees encountered while traversing this map given a slope. The map
    /// repeats infinitely to the right, but not downwards.
    ///
    /// # Examples
    ///
    /// ```
    /// let map = Map::new(vec![
    ///     String::from("#.."),
    ///     String::from(".#."),
    ///     String::from("..#"),
    /// ])
    /// .expect("cannot construct map");
    /// assert_eq!(1, map.count_trees());
    /// ```
    fn count_trees<'a>(&self, slope: Slope) -> usize {
        let mut pos = slope;
        let mut count = 0;
        while pos.0 < self.height {
            // Since pos.0 < self.height and pos.1 < self.width, we can safely use them as indices
            if self.map[pos.0].chars().nth(pos.1).unwrap() == '#' {
                count += 1;
            }

            pos.0 += slope.0;
            pos.1 = (pos.1 + slope.1) % self.width;
        }
        count
    }
}

/// Solves the puzzle for day 3 of 2020.
pub fn solve(input: &str) -> solver::Result {
    let mut solutions = Vec::new();

    let map = Map::new(input.lines().map(String::from).collect())?;
    solutions.push(map.count_trees((1, 3)).to_string());
    solutions.push(
        [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
            .iter()
            .map(|slope| map.count_trees(*slope))
            .product::<usize>()
            .to_string(),
    );

    Ok(solutions)
}
