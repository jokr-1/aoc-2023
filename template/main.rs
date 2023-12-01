fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn part_one(input: &str) -> i32 {
    0
}

fn part_two(input: &str) -> i32 {
    0
}

#[test]
fn part1() {
    let input = include_str!("../testinput-1.txt");
    assert_eq!(part_one(input), 1);
}

#[test]
fn part2() {
    let input = include_str!("../testinput-2.txt");
    assert_eq!(part_two(input), 1);
}
