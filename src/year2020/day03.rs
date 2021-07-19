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

    /// Counts the number of trees encountered while traversing this map using slopes provided by an
    /// iterator. Counting is stopped once either the bottom of this map is reached or the iterator
    /// of slopes is exhausted.
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
    fn count_trees<'a>(&self, slope_iter: impl Iterator<Item = &'a Slope>) -> usize {
        let mut slope_iter = slope_iter.peekable();

        let mut pos = (0, 0);
        let mut count = 0;
        loop {
            let slope = slope_iter.next().unwrap();
            pos.0 += slope.0;
            pos.1 = (pos.1 + slope.1) % self.width;
            if pos.0 >= self.height && slope_iter.peek().is_none() {
                break;
            }

            // Since pos.0 < self.height and pos.1 < self.width, we can safely use them as indices
            if self.map[pos.0].chars().nth(pos.1).unwrap() == '#' {
                count += 1;
            }
        }

        count
    }
}

/// Solves the puzzle for day 3 of 2020.
pub fn solve(input: &str) -> solver::Result {
    let mut solutions = Vec::new();

    let map = Map::new(input.lines().map(String::from).collect())?;
    solutions.push(map.count_trees([(1, 3)].iter().cycle()).to_string());

    Ok(solutions)
}
