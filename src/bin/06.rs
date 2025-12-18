advent_of_code::solution!(6);

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            x => Err(format!("Could not parse {x}")),
        }
    }

    fn exec(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let num_lines = lines.len();
    let mut cols: Vec<Vec<u64>> = vec![];
    let mut operations: Vec<Operation> = vec![];

    for i in 0..num_lines {
        let line = lines[i];
        let items: Vec<&str> = line.split_ascii_whitespace().collect();

        if i == num_lines - 1 {
            operations = items.iter().map(|o| Operation::parse(o).unwrap()).collect();
        } else {
            for (j, item) in items.iter().enumerate() {
                let num = item.parse().unwrap();
                if let Some(nums) = cols.get_mut(j) {
                    nums.push(num);
                } else {
                    cols.push(vec![num]);
                }
            }
        }
    }

    let mut totals: Vec<u64> = vec![];
    for (i, nums) in cols.into_iter().enumerate() {
        let operation = operations.get(i).unwrap();
        totals.push(
            nums.into_iter()
                .reduce(|n, acc| operation.exec(n, acc))
                .unwrap(),
        );
    }

    let total: u64 = totals.iter().sum();
    Some(total)
}

fn digits_to_number(digits: &[Option<String>]) -> u64 {
    let digits: Vec<u64> = digits
        .into_iter()
        .filter_map(|digit| digit.as_ref().map(|d| d.parse::<u64>().unwrap()))
        .collect();

    let mut n = 0;
    for d in digits {
        n = n * 10 + d;
    }
    n
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().peekable();
    let num_rows = lines.clone().count() - 1 as usize;
    let num_cols = lines.peek()?.len();

    let operations: Vec<Operation> = lines
        .next_back()
        .unwrap()
        .split_ascii_whitespace()
        .map(|o| Operation::parse(o).unwrap())
        .rev()
        .collect();

    let rows: Vec<Vec<Option<String>>> = lines
        .map(|line| {
            line.chars()
                .rev()
                .map(|char| {
                    if char.is_whitespace() {
                        None
                    } else {
                        Some(char.to_string())
                    }
                })
                .collect()
        })
        .collect();

    let mut vsi = 0;
    let mut vertical_slices: Vec<Vec<Vec<Option<String>>>> = vec![];
    for ci in 0..num_cols {
        let mut digits = vec![];
        for ri in 0..num_rows {
            let digit = rows[ri][ci].clone();
            digits.push(digit);
        }

        if digits.iter().all(Option::is_none) {
            vsi += 1;
            continue;
        }

        if vertical_slices.len() <= vsi {
            vertical_slices.resize_with(vsi + 1, Vec::new);
        }

        vertical_slices[vsi].push(digits);
    }

    let mut totals: Vec<u64> = vec![];
    for (i, slice) in vertical_slices.iter().enumerate() {
        let operation = operations.get(i).unwrap();
        let nums: Vec<u64> = slice
            .iter()
            .map(|digits| digits_to_number(digits))
            .collect();

        totals.push(
            nums.into_iter()
                .reduce(|n, acc| operation.exec(n, acc))
                .unwrap(),
        );
    }

    let total: u64 = totals.iter().sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
