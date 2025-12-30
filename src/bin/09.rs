use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

advent_of_code::solution!(9);

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn parse(line: &str) -> Self {
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        Self::new(x, y)
    }
}

fn points_in_row(row: &i64, rows: &BTreeMap<i64, BTreeSet<i64>>) -> Vec<Point> {
    rows.get(row)
        .unwrap()
        .iter()
        .map(|col| Point::new(*col, *row))
        .collect()
}

fn calc_areas((row1, row2): (&i64, &i64), rows: &BTreeMap<i64, BTreeSet<i64>>) -> Vec<u64> {
    let r1s = points_in_row(row1, &rows);
    let r2s = points_in_row(row2, &rows);
    r1s.iter()
        .flat_map(|a| r2s.iter().map(|b| calc_area(a, b)))
        .collect()
}

fn calc_area(a: &Point, b: &Point) -> u64 {
    let dx = (a.x - b.x).unsigned_abs() as u128 + 1;
    let dy = (a.y - b.y).unsigned_abs() as u128 + 1;
    (dx * dy) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let rows: BTreeMap<i64, BTreeSet<i64>> =
        input.lines().fold(BTreeMap::new(), |mut acc, line| {
            let point = Point::parse(line);
            acc.entry(point.y).or_default().insert(point.x);
            acc
        });

    let areas: Vec<u64> = rows
        .keys()
        .into_iter()
        .tuple_combinations()
        .flat_map(|(row1, row2)| calc_areas((row1, row2), &rows))
        .sorted_by(|a, b| b.cmp(&a))
        .collect();

    let area = *areas.first()?;
    Some(area)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
