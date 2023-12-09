fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|list| {
            let mut tails = Vec::new();
            let mut current = list.clone();

            while !current.iter().all(|&v| v == 0) {
                tails.push(current.last().cloned().unwrap());
                let diff = current
                    .clone()
                    .iter()
                    .zip(current[1..].iter())
                    .map(|(a, b)| b - a)
                    .collect::<Vec<i32>>();
                current = diff;
            }

            tails.iter().sum::<i32>()
        })
        .sum()
}

fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|list| {
            let mut heads = Vec::new();
            let mut current = list.clone();

            while !current.iter().all(|&v| v == 0) {
                heads.push(current.first().cloned().unwrap());
                let diff = current
                    .clone()
                    .iter()
                    .zip(current[1..].iter())
                    .map(|(a, b)| a - b)
                    .collect::<Vec<i32>>();
                current = diff;
            }

            heads.iter().sum::<i32>()
        })
        .sum()
}
