use std::{collections::HashSet, str::FromStr};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

struct Universe {
    galaxies: Vec<(usize, usize)>,
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
}

impl Universe {
    fn distance(&self, a: usize, b: usize, expansion_factor: usize) -> usize {
        let a = self.galaxies[a];
        let b = self.galaxies[b];

        let (r_start, r_end) = (a.0.min(b.0), a.0.max(b.0));
        let (c_start, c_end) = (a.1.min(b.1), a.1.max(b.1));

        let dr = (r_start..r_end)
            .filter(|r| self.empty_rows.contains(r))
            .count();
        let dc = (c_start..c_end)
            .filter(|c| self.empty_cols.contains(c))
            .count();

        (b.0 as i32 - a.0 as i32).unsigned_abs() as usize
            + (b.1 as i32 - a.1 as i32).unsigned_abs() as usize
            + (dr + dc) * (expansion_factor - 1)
    }

    fn all_distances(&self, expansion_factor: usize) -> usize {
        let n = self.galaxies.len();
        (0..n - 1)
            .flat_map(|i| (i + 1..n).map(move |j| self.distance(i, j, expansion_factor)))
            .sum()
    }
}

impl FromStr for Universe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (n, m) = (s.lines().count(), s.lines().next().unwrap().chars().count());

        let galaxies: Vec<(usize, usize)> = s
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars().enumerate().filter_map(move |(j, c)| match c {
                    '#' => Some((i, j)),
                    _ => None,
                })
            })
            .collect();

        let xs = galaxies.iter().map(|&pos| pos.0).collect::<HashSet<_>>();
        let ys = galaxies.iter().map(|&pos| pos.1).collect::<HashSet<_>>();

        let empty_rows: HashSet<usize> = (0..n).filter(|r| !xs.contains(r)).collect();
        let empty_cols: HashSet<usize> = (0..m).filter(|r| !ys.contains(r)).collect();

        Ok(Universe {
            galaxies,
            empty_rows,
            empty_cols,
        })
    }
}

fn part_one(input: &str) -> usize {
    let universe: Universe = input.parse().unwrap();
    universe.all_distances(2)
}

fn part_two(input: &str) -> usize {
    let universe: Universe = input.parse().unwrap();
    universe.all_distances(1_000_000)
}
