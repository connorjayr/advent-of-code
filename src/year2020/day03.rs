use crate::solver;
use std::vec::Vec;

/// A slope, which represents a step on a two-dimensional map. The first value corresponds to
/// horizontal movement whereas the second value corresponds to vertical movement.
type Slope = (i32, i32);

/// A two-dimensional map of the area, where `#` represents a tree and `.` represents an open
/// square.
struct Map {
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
    pub fn new(map: Vec<String>) -> Result<Self, &'static str> {
        let width = map.get(0).unwrap_or(&String::from("")).len();
        if map.iter().all(|row| row.len() == width) {
            Ok(Map { map })
        } else {
            Err("map must be rectangular")
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
    fn count_trees(&self, slope_iter: impl Iterator<Item = Slope>) -> usize {
        let slope_iter = slope_iter.peekable();

        let mut pos = (0, 0);
        let mut count = 0;
        while pos.1 < self.map.len() && slope_iter.peek().is_some() {
            let slope = slope_iter.next().unwrap();
            pos.0 += slope.0;
            pos.1 += slope.1;
        }

        count
    }
}

/// Solves the puzzle for day 3 of 2020.
pub fn solve(input: &str) -> solver::Result {
    let mut solutions = Vec::new();

    Ok(solutions)
}
