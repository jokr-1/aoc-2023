fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let numbers = line.chars().filter(|c| c.is_numeric());
            let mut d = numbers.clone().next().unwrap().to_string();
            d.push(numbers.last().unwrap());
            d.parse::<i32>().unwrap()
        })
        .sum()
}

const NUM_STR: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn first_number(line: &str, reversed: bool) -> Option<String> {
    let n = line.len();

    for i in 0..n {
        for (&p, x) in NUM_STR.iter().zip(1..10) {
            let num = x.to_string();
            let idx = if reversed { n - i - 1 } else { i };
            if line[idx..].starts_with(p) || line[idx..].starts_with(&num) {
                return Some(num);
            }
        }
    }
    None
}

fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut res = String::new();
            res += &first_number(line, false).unwrap();
            res += &first_number(line, true).unwrap();
            res.parse::<i32>().unwrap()
        })
        .sum()
}

#[test]
fn part1() {
    let input = include_str!("../testinput-1.txt");
    assert_eq!(part_one(input), 142);
}

#[test]
fn part2() {
    let input = include_str!("../testinput-2.txt");
    assert_eq!(part_two(input), 281);
}
