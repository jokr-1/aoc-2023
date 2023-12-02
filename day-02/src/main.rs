use std::{str::FromStr, cmp::max};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

struct Game {
    id: i32,
    bags: Vec<Bag>,
}

impl FromStr for Game {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, body) = s.split_once(':').unwrap();
        let id = prefix.strip_prefix("Game ").unwrap().parse::<i32>().unwrap();
        let bags = body.split(';').map(|input| {
            input.parse::<Bag>().unwrap()
        }).collect();
        Ok(Game { id, bags })
    }
}

struct Bag  {
    red: i32,
    green: i32,
    blue: i32
}

impl FromStr for Bag {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bag = Bag { red: 0, green: 0 , blue: 0};
        for chunk in s.split(',') {
            let (num, color) = chunk.trim().split_once(' ').unwrap();
            let num = num.parse::<i32>().unwrap();
            match color {
                "red" => bag.red += num,
                "green" => bag.green += num,
                "blue"=> bag.blue += num,
                _ => return Err("Unable to parse.".to_string())
            }
        }
        Ok(bag)
    }
}

fn part_one(input: &str) -> i32 {
    input.lines().filter_map(|line| {
        let game = line.parse::<Game>().unwrap();
        let invalid = game.bags.iter().any(|bag| {
            bag.red > 12 || bag.green > 13 || bag.blue > 14
        });
        if invalid { None } else { Some(game.id) }
    }).sum()
}

fn part_two(input: &str) -> i32 {
    input.lines().map(|line| {
        let game = line.parse::<Game>().unwrap();
        let min = game.bags.iter().fold((0,0,0), |acc, bag| {
            (max(acc.0, bag.red), max(acc.1, bag.green), max(acc.2, bag.blue))
        });
        min.0*min.1*min.2
    }).sum()
}