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
    let mut lines = input.lines().peekable();
    let last_row = lines.clone().count() - 2 as usize;
    let last_col = lines.peek()?.chars().count() - 1 as usize;

    let origin = lines.next()?.chars().enumerate().find_map(|(col, char)| {
        if char == 'S' {
            Some(Point::new(0, col))
        } else {
            None
        }
    })?;

    let manifold: Vec<Vec<bool>> = lines
        .map(|line| {
            line.chars()
                .map(|char| if char == '^' { true } else { false })
                .collect()
        })
        .collect();

    let mut current_row = HashMap::from([(origin, 1 as u64)]);
    for _ in 1..last_row {
        let mut next_row: HashMap<Point, u64> = HashMap::new();
        for (beam, count) in current_row.into_iter() {
            let next_down = Point {
                row: beam.row + 1,
                col: beam.col,
            };

            if manifold[next_down.row][next_down.col] {
                if let Some(col) = next_down.col.checked_sub(1) {
                    let split = Point {
                        row: next_down.row,
                        col,
                    };
                    *next_row.entry(split).or_insert(0) += count;
                }

                if next_down.col + 1 <= last_col {
                    let split = Point {
                        row: next_down.row,
                        col: next_down.col + 1,
                    };
                    *next_row.entry(split).or_insert(0) += count;
                };
            } else {
                *next_row.entry(next_down).or_insert(0) += count;
            }
        }
        current_row = next_row;
    }

    let total = current_row.iter().fold(0, |acc, (_, count)| acc + count);
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
