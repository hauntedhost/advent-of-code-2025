use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn next_down(&self) -> Self {
        Self {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn next_split(&self) -> [Self; 2] {
        let row = self.row + 1;
        [
            Self {
                row,
                col: self.col - 1,
            },
            Self {
                row,
                col: self.col + 1,
            },
        ]
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let origin = lines.next()?.chars().enumerate().find_map(|(col, char)| {
        if char == 'S' {
            Some(Point::new(0, col))
        } else {
            None
        }
    })?;

    let splitters = lines
        .enumerate()
        .fold(HashSet::new(), |mut acc, (row, line)| {
            for (col, char) in line.chars().enumerate() {
                if char == '^' {
                    acc.insert(Point::new(row, col));
                }
            }
            acc
        });

    let last_row = splitters.len() - 1;
    let mut beams = HashSet::from([origin]);
    let mut splits: Vec<Point> = vec![];
    let mut end_of_manifold = false;

    while !end_of_manifold {
        beams = beams.iter().fold(HashSet::new(), |mut acc, beam| {
            if splitters.contains(&beam.next_down()) {
                splits.push(beam.clone());
                acc.extend(beam.next_split());
            } else {
                acc.insert(beam.next_down());
            }
            acc
        });

        if beams.iter().all(|beam| beam.row == last_row) {
            end_of_manifold = true;
        }
    }

    let split_count = splits.len() as u64;
    Some(split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].chars().count();

    let origin_col = lines[0].chars().position(|c| c == 'S')?;

    let manifold: Vec<Vec<bool>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c == '^').collect())
        .collect();

    let mut current = HashMap::from([(origin_col, 1u64)]);
    for row_i in 0..(rows - 1) {
        let mut next: HashMap<usize, u64> = HashMap::new();
        for (col_i, count) in current.into_iter() {
            let next_row_i = row_i + 1;
            if manifold[next_row_i][col_i] {
                if let Some(next_col_i) = col_i.checked_sub(1) {
                    *next.entry(next_col_i).or_insert(0) += count;
                }

                if col_i + 1 < cols {
                    *next.entry(col_i + 1).or_insert(0) += count;
                }
            } else {
                *next.entry(col_i).or_insert(0) += count;
            }
        }
        current = next;
    }

    let total = current.values().copied().sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
