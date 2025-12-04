advent_of_code::solution!(4);

const ADJACENTS: [(isize, isize); 8] = [
    (-1, 0),  // ⬆️
    (-1, 1),  // ↗️
    (0, 1),   // ➡️
    (1, 1),   // ↘️
    (1, 0),   // ⬇️
    (1, -1),  // ↙️
    (0, -1),  // ⬅️
    (-1, -1), // ↖️
];

fn get_cell(grid: &Vec<Vec<String>>, i: isize, j: isize) -> Option<&String> {
    if i < 0 || j < 0 {
        return None;
    }
    let (i, j) = (i as usize, j as usize);
    grid.get(i)?.get(j)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for char in line.chars() {
            row.push(char.to_string());
        }
        grid.push(row);
    }

    let mut rolls: u64 = 0;
    for (i, row) in grid.iter().enumerate() {
        'cells: for (j, cell) in row.iter().enumerate() {
            if cell == "." {
                continue;
            };

            let mut a_rolls = 0;
            for (ai, aj) in ADJACENTS {
                let ci = i as isize + ai;
                let cj = j as isize + aj;
                if let Some(cell) = get_cell(&grid, ci, cj)
                    && cell == "@"
                {
                    a_rolls += 1;
                    if a_rolls > 3 {
                        continue 'cells;
                    }
                } else {
                    continue;
                }
            }
            rolls += 1;
        }
    }

    Some(rolls)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
