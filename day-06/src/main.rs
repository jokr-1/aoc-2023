fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn solve(duration: u64, distance: u64) -> u64 {
    let a = (duration as f64) / 2.0;
    let b = f64::sqrt(a.powf(2.0) - (distance as f64 + 1e-9));
    (a + b).floor() as u64 - (a - b).ceil() as u64 + 1
}

fn part_one(input: &str) -> u64 {
    let data = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|c| c.parse::<u64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let times = data[0].clone();
    let distance = data[1].clone();

    times
        .iter()
        .zip(distance)
        .fold(1, |acc, (duration, dist)| acc * solve(*duration, dist))
}

fn part_two(input: &str) -> u64 {
    let data = input
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .fold(0u64, |acc, i| acc * 10 + i as u64)
        })
        .collect::<Vec<_>>();

    solve(data[0], data[1])
}
