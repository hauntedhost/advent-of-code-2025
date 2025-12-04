advent_of_code::solution!(3);

#[derive(Debug)]
struct Pair {
    left: u64,
    right: u64,
}

impl Pair {
    fn new(left: u64, right: u64) -> Self {
        Self { left, right }
    }

    fn concat(&self) -> u64 {
        self.left * 10_u64.pow(num_digits(self.right) as u32) + self.right
    }
}

fn num_digits(n: u64) -> usize {
    n.to_string().len()
}

fn parse_digits(s: &str) -> Vec<u64> {
    s.chars().map(|c| c.to_string().parse().unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut joltage: u64 = 0;

    for line in input.lines() {
        let digits = parse_digits(line);
        let mut pair = Pair::new(0, 0);

        for i in 0..digits.len() - 1 {
            let left = digits[i];
            if left < pair.left {
                continue;
            };

            if left > pair.left {
                pair = Pair::new(left, 0);
            };

            for j in i + 1..digits.len() {
                let right = digits[j];
                if right < pair.right {
                    continue;
                }

                if right > pair.right {
                    pair.right = right;
                };
            }
        }
        joltage += pair.concat();
    }
    Some(joltage)
}

#[derive(Debug)]
struct Battery {
    digit: u64,
    index: usize,
}

impl Battery {
    fn new(digit: u64, index: usize) -> Self {
        Self { digit, index }
    }

    fn concat_digits(batteries: &[Battery]) -> u64 {
        let mut n = 0;
        for b in batteries {
            n = n * 10 + b.digit;
        }
        n
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut joltage: u64 = 0;

    for line in input.lines() {
        let digits = parse_digits(line);
        let mut acc: Vec<Battery> = vec![];

        while acc.len() < 12 {
            let start_index = acc.last().map_or(0, |b| b.index + 1);
            let max_index = (digits.len() - 12) + acc.len();
            let mut next_battery = Battery::new(0, 0);

            for i in start_index..=max_index {
                let digit = digits[i];
                if digit > next_battery.digit {
                    next_battery = Battery::new(digit, i);
                };
            }
            acc.push(next_battery);
        }
        joltage += Battery::concat_digits(&acc);
    }
    Some(joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
