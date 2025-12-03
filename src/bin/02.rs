use std::ops::RangeInclusive;

advent_of_code::solution!(2);

fn parse_range(range: &str) -> Result<RangeInclusive<u64>, String> {
    let (a, b) = range
        .split_once('-')
        .ok_or(format!("split error on range {range}"))?;

    let start: u64 = a.parse().map_err(|_| format!("parse int error {a}"))?;
    let end: u64 = b.parse().map_err(|_| format!("parse int error {b}"))?;

    Ok(start..=end)
}

fn parse_ranges(input: &str) -> Result<Vec<RangeInclusive<u64>>, String> {
    input
        .trim()
        .split(',')
        .map(|range| parse_range(range))
        .collect()
}

fn is_odd(n: usize) -> bool {
    n % 2 != 0
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut invalid_ids: Vec<u64> = vec![];

    for range in parse_ranges(&input).unwrap() {
        for n in range {
            let s = n.to_string();
            let s_len = s.len();
            if is_odd(s_len) {
                continue;
            };
            let (first, last) = s.split_at(s_len / 2);
            if first == last {
                invalid_ids.push(n);
            }
        }
    }

    let invalid_sum = invalid_ids.iter().sum();
    Some(invalid_sum)
}

// build range for block_len, e.g. 1 => 1..=9, 2 => 10.=99, etc
fn block_range(block_len: usize) -> RangeInclusive<u64> {
    let start = 10_u64.pow((block_len as u32) - 1);
    let end = 10_u64.pow(block_len as u32) - 1;
    start..=end
}

// generate all invalid ids with repeating patterns up to max_id
fn generate_invalid_ids(max_id: u64) -> Vec<u64> {
    let mut invalid_ids = vec![];
    let max_digits = max_id.to_string().len();
    for total_digits in 2..=max_digits {
        let midpoint = total_digits / 2;
        for block_len in 1..=midpoint {
            if total_digits % block_len != 0 {
                continue;
            }

            let repeats = total_digits / block_len;
            if repeats < 2 {
                continue;
            }

            for block in block_range(block_len) {
                let invalid_id: u64 = block.to_string().repeat(repeats).parse().unwrap();
                if invalid_id > max_id {
                    break;
                }
                invalid_ids.push(invalid_id);
            }
        }
    }

    invalid_ids.sort_unstable();
    invalid_ids.dedup();
    invalid_ids
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_ranges(&input).unwrap();
    let max_id = ranges.iter().map(|r| *r.end()).max().unwrap();
    let invalid_ids = generate_invalid_ids(max_id);

    let mut invalid_sum: u64 = 0;
    for range in ranges {
        let (start, end) = range.into_inner();
        let lo = invalid_ids.partition_point(|&x| x < start);
        let hi = invalid_ids.partition_point(|&x| x <= end);
        invalid_sum += invalid_ids[lo..hi].iter().sum::<u64>();
    }

    Some(invalid_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
