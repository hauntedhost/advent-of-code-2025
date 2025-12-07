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

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
