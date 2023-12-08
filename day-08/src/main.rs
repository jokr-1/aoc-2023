use core::panic;
use std::collections::HashMap;

type Graph = HashMap<String, (String, String)>;

fn main() {
    let input = include_str!("../input.txt");
    let (instructions, body) = input.split_once("\n\n").unwrap();
    let graph: Graph = body
        .lines()
        .map(|l| {
            let (node, edges) = l.split_once(" = ").unwrap();
            let (left, right) = edges
                .trim_matches('(')
                .trim_matches(')')
                .split_once(", ")
                .unwrap();
            (node.to_string(), (left.to_string(), right.to_string()))
        })
        .collect();

    // println!("Part 1: {:?}", part_one(instructions, &graph));
    println!("Part 2: {:?}", part_two(instructions, &graph));
}

fn part_one(instructions: &str, graph: &Graph) -> usize {
    let mut next = "AAA".to_string();

    for (step, i) in instructions.chars().cycle().enumerate() {
        if next == "ZZZ".to_string() {
            return step;
        }

        next = match i {
            'L' => graph[&next.clone()].0.clone(),
            'R' => graph[&next.clone()].1.clone(),
            _ => panic!("Unknwn instruction."),
        };
    }
    0
}

fn part_two(instructions: &str, graph: &Graph) -> usize {
    let starts: Vec<String> = graph.keys().cloned().filter(|k| k.ends_with("A")).collect();

    let counts = starts
        .iter()
        .cloned()
        .map(|start| {
            let mut next = start;
            for (step, i) in instructions.chars().cycle().enumerate() {
                if next.ends_with("Z") {
                    return step;
                }
                next = match i {
                    'L' => graph[&next].0.clone(),
                    'R' => graph[&next].1.clone(),
                    _ => panic!("Unknwn instruction."),
                };
            }
            0
        })
        .collect::<Vec<_>>();

    counts.iter().fold(1, |acc, &i| lcm(acc, i))
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
