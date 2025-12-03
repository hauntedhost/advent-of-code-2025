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

fn divisors(n: u64) -> Vec<u64> {
    let mut divs = Vec::new();
    let mut div = 1;

    while div * div <= n {
        if n % div == 0 {
            if div != n {
                divs.push(div);
            }

            let div_pair = n / div;
            if div_pair != div && div_pair != n && div_pair != 1 {
                divs.push(div_pair);
            }
        }
        div += 1;
    }

    divs
}

fn chunk_str(s: &str, n: usize) -> Vec<String> {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(n)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

fn any_not_equal<T: PartialEq>(v: &[T]) -> bool {
    if let Some(first) = v.first() {
        v.iter().skip(1).any(|x| x != first)
    } else {
        false
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut invalid_ids: Vec<u64> = vec![];

    for range in parse_ranges(&input).unwrap() {
        'n: for n in range {
            let s = n.to_string();
            let s_len = s.len();

            for div in divisors(s_len as u64) {
                let s_chunks = chunk_str(&s, div as usize);
                if !any_not_equal(&s_chunks) {
                    invalid_ids.push(n);
                    continue 'n;
                }
            }
        }
    }

    let invalid_sum = invalid_ids.iter().sum();
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
