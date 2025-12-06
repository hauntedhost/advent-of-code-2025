use std::cmp::{max, min};
use std::ops::RangeInclusive;

advent_of_code::solution!(5);

fn parse_ranges(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .lines()
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            start..=end
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges = parse_ranges(ranges);

    let mut fresh_ids = vec![];
    'ids: for id in ids.lines() {
        let id: u64 = id.parse().unwrap();
        for range in &ranges {
            if range.contains(&id) {
                fresh_ids.push(id);
                continue 'ids;
            }
        }
    }

    let fresh_count = fresh_ids.len() as u64;
    Some(fresh_count)
}

fn flatten_ranges(ranges: &mut Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    for i in 0..ranges.len() - 1 {
        let r1 = ranges[i].clone();

        for j in i + 1..ranges.len() {
            let r2 = ranges[j].clone();

            if r1.start() <= r2.end() && r2.start() <= r1.end() {
                let start = min(r1.start(), r2.start()).clone();
                let end = max(r1.end(), r2.end()).clone();
                let flat_range = start..=end;

                let min_i = min(i, j);
                let max_i = max(i, j);
                ranges[min_i] = flat_range;
                ranges.remove(max_i);

                return flatten_ranges(ranges);
            }
        }
    }

    ranges.clone()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let mut ranges = parse_ranges(ranges);

    flatten_ranges(&mut ranges);

    let fresh_count = ranges.into_iter().fold(0, |acc, r| acc + r.count());
    Some(fresh_count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
