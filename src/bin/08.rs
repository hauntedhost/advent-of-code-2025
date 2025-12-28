use itertools::Itertools;
use std::mem::swap;

advent_of_code::solution!(8);

type Point = (i64, i64, i64);

fn parse_point(line: &str) -> Point {
    let coords: Vec<i64> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
    (coords[0], coords[1], coords[2])
}

struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb {
            return false;
        }

        // union by size
        if self.size[ra] < self.size[rb] {
            swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        for i in 0..n {
            let _ = self.find(i);
        }

        let mut seen = vec![false; n];
        let mut sizes = vec![];
        for i in 0..n {
            let r = self.parent[i];
            if !seen[r] {
                seen[r] = true;
                sizes.push(self.size[r]);
            }
        }
        sizes
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let junctions: Vec<Point> = input.lines().map(parse_point).collect();
    let num_junctions = junctions.len();

    // take 10 pairs for example input, 1000 for full input
    let num_pairs = if num_junctions == 20 { 10 } else { 1000 };

    // build an ordered list of distances between every unique pair
    let edges: Vec<(i128, usize, usize)> = (0..num_junctions)
        .tuple_combinations()
        .map(|(i, j)| {
            let (ax, ay, az) = junctions[i];
            let (bx, by, bz) = junctions[j];
            let dist: i128 =
                (ax - bx).pow(2) as i128 + (ay - by).pow(2) as i128 + (az - bz).pow(2) as i128;
            (dist, i, j)
        })
        .sorted_by(|(dist1, _, _), (dist2, _, _)| dist1.cmp(dist2))
        .collect();

    // create a disjoint set with a union of every pair
    let mut dset = DisjointSet::new(num_junctions);
    for &(_, i, j) in edges.iter().take(num_pairs) {
        dset.union(i, j);
    }

    let mut sizes = dset.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a)); // desc

    let top3 = sizes.into_iter().take(3).map(|x| x as u64).product();
    Some(top3)
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
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
