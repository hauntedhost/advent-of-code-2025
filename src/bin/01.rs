advent_of_code::solution!(1);

fn parse_rotation(line: &str) -> Result<i64, String> {
    let (first, rest) = line.split_at(1);

    let dist: i64 = rest
        .parse()
        .map_err(|_| format!("invalid distance {rest}"))?;

    match first {
        "L" => Ok(-dist),
        "R" => Ok(dist),
        _ => Err(format!("invalid direction: {first}")),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut position: i64 = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let rotation = parse_rotation(&line).unwrap();
        position = (position + rotation).rem_euclid(100);
        if position == 0 {
            zero_count += 1;
        };
    }

    Some(zero_count)
}

fn count_zero_hits(position: u64, rotation: i64) -> u64 {
    if rotation == 0 {
        return 0;
    }

    let total_clicks = rotation.unsigned_abs();

    if position == 0 {
        total_clicks / 100
    } else {
        let clicks_to_zero = if rotation > 0 {
            100 - position
        } else {
            position
        };

        if total_clicks < clicks_to_zero {
            0
        } else {
            1 + (total_clicks - clicks_to_zero) / 100
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut position: i64 = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let rotation = parse_rotation(&line).unwrap();
        zero_count += count_zero_hits(position as u64, rotation);
        position = (position + rotation).rem_euclid(100);
    }

    Some(zero_count)
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
        assert_eq!(result, Some(6));
    }
}
