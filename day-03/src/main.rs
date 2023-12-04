use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

enum Entity {
    Number(u8),
    Symbol(char),
    None,
}

struct Index {
    row: usize,
    col: usize,
}

type Schematic = Vec<Vec<Entity>>;
// 
impl FromStr for <Vec<Vec<Entity>>> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Part {
    id: u32,
    loc: (usize, usize),
    length: usize,
}


impl Part {

    fn neighbors(&self, map: &Schematic) -> Vec<(usize, usize)> {
        let rows = map.len();
        let cols = map[0].len();
    
        let left = [(-1,-1),(0,-1),(1,-1)];
        let right = [(-1,1),(0,1),(1,1)];
        let mid = [(-1,0),(1,0)];

        (0..self.length).flat_map(|i| {
            mid.map(|(dr,dc)| {
                 (self.loc.0 as i32 + dr, self.loc.1 as i32 + dc + i as i32)
            })
        }).chain(left.map(|(dr,dc)| {
            (self.loc.0 as i32 + dr, self.loc.1 as i32 + dc)
        })).chain(right.map(|(dr,dc)| {
            (self.loc.0 as i32 + dr, self.loc.1 as i32 + dc + self.length as i32 -1 )
        })).filter_map(|(row,col)| {
            if row >= 0 && row < rows as i32 && col >= 0 && col < cols as i32{
                Some((row as usize, col as usize))
            } else {
                None
            }
        }).collect::<Vec<(usize, usize)>>()
        
    }


    fn is_valid(&self, map: &Schematic) -> bool {
        self.neighbors(&map).iter().any(|(row, col)| {
            let s = map[*row][*col];
            match s {
                x if x.is_numeric() => false,
                x if x == '.' => false,
                _ => true
            }
        })

    }
}

type Schematic = Vec<Vec<char>>;

fn get_parts(input: &str) -> Vec<Part> {
    let schematic: Schematic = input.lines().map(|line| line.chars().collect()).collect();
    let mut numbers = Vec::new();
    for (row, line) in input.lines().enumerate() {
        let mut current: Option<Part> = None;
        for (col, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap();
                if let Some(ref mut candidate) = current {
                    candidate.id = 10*candidate.id + digit;
                    candidate.length += 1;
                } else {
                    current = Some(Part {
                        id: digit,
                        loc: (row, col),
                        length: 1,
                    });
                }
            }
            else {
                if current.is_some() {
                    numbers.push(current.take().unwrap());
                }
            }
        }
        if current.is_some() {
            numbers.push(current.take().unwrap());
        }
    }
    numbers.iter().cloned().filter(|n| n.is_valid(&schematic)).map(|c| c).collect::<Vec<Part>>()
}

fn part_one(input: &str) -> u32 {
    get_parts(input).iter().map(|p| p.id).sum::<u32>()
}

fn part_two(input: &str) -> u32 {
    let schematic: Schematic = input.lines().map(|line| line.chars().collect()).collect();

    let gear_candidates = input.lines().enumerate().flat_map(|(row, line)| {
        line.chars().enumerate().filter_map(move |(col, c)| match c {
            '*' => Some((row,col)),
            _ => None
        })})
        .collect::<Vec<(usize, usize)>>();
    let parts = get_parts(input);

    gear_candidates.iter().map(|g| {
        parts.iter().cloned().filter(|p| p.neighbors(&schematic).iter().any(|n| n.0==g.0 && n.1 == g.1)).collect::<Vec<Part>>()
    }).filter(|p| p.len() == 2).map(|v| v.iter().map(|p| p.id).product::<u32>()).sum()
}