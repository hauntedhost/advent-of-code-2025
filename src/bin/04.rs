advent_of_code::solution!(4);

const NEIGHBOR_DELTAS: [(isize, isize); 8] = [
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

    let mut total_rolls: u64 = 0;
    for (i, row) in grid.iter().enumerate() {
        'cells: for (j, cell) in row.iter().enumerate() {
            if cell == "." {
                continue;
            };

            let mut rolls = 0;
            for (dx, dy) in NEIGHBOR_DELTAS {
                let ni = i as isize + dx;
                let nj = j as isize + dy;
                if let Some(cell) = get_cell(&grid, ni, nj)
                    && cell == "@"
                {
                    rolls += 1;
                    if rolls > 3 {
                        continue 'cells;
                    }
                } else {
                    continue;
                }
            }
            total_rolls += 1;
        }
    }

    Some(total_rolls)
}

fn take_rolls(grid: &mut Vec<Vec<String>>) -> Option<u64> {
    let mut rolls_taken: u64 = 0;
    for (i, row) in grid.clone().iter().enumerate() {
        'cells: for (j, cell) in row.iter().enumerate() {
            if cell == "." {
                continue;
            };

            let mut rolls = 0;
            for (dx, dy) in NEIGHBOR_DELTAS {
                let ni = i as isize + dx;
                let nj = j as isize + dy;
                if let Some(cell) = get_cell(&grid, ni, nj)
                    && cell == "@"
                {
                    rolls += 1;
                    if rolls > 3 {
                        continue 'cells;
                    }
                } else {
                    continue;
                }
            }
            grid[i][j] = ".".to_string();
            rolls_taken += 1;
        }
    }

    if rolls_taken > 0 {
        Some(rolls_taken)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for char in line.chars() {
            row.push(char.to_string());
        }
        grid.push(row);
    }

    let mut total_rolls: u64 = 0;
    while let Some(rolls_taken) = take_rolls(&mut grid) {
        total_rolls += rolls_taken;
    }

    Some(total_rolls)
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
        assert_eq!(result, Some(43));
    }
}
