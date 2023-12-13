fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

#[derive(Debug, PartialEq)]
enum Item {
    Pipe(Connection, Connection),
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Connection {
    North,
    East,
    South,
    West,
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '|' => Item::Pipe(Connection::North, Connection::South),
            '-' => Item::Pipe(Connection::West, Connection::East),
            'L' => Item::Pipe(Connection::North, Connection::East),
            'J' => Item::Pipe(Connection::North, Connection::West),
            '7' => Item::Pipe(Connection::West, Connection::South),
            'F' => Item::Pipe(Connection::South, Connection::East),
            '.' => Item::Ground,
            'S' => Item::Start,
            other => panic!("Unkown symbold {other}"),
        }
    }
}

fn next(
    map: &Vec<Vec<Item>>,
    pos: (usize, usize),
    from: &Connection,
) -> Option<((usize, usize), Connection)> {
    if let Item::Pipe(start, end) = &map[pos.0][pos.1] {
        let to = if start == from { end } else { start };
        let (delta, check) = match to {
            Connection::East => ((0, 1), Connection::West),
            Connection::West => ((0, -1i16), Connection::East),
            Connection::North => ((-1i16, 0), Connection::South),
            Connection::South => ((1, 0), Connection::North),
        };

        let (x, y) = (pos.0 as i16 + delta.0, pos.1 as i16 + delta.1);
        let (n, m) = (map.len(), map[0].len());

        if x < 0 || y < 0 || x > n as i16 || y > m as i16 {
            return None;
        }

        let pos_next = (x as usize, y as usize);

        // check next element
        if let Item::Pipe(start, end) = &map[pos_next.0][pos_next.1] {
            if start == &check {
                Some((pos_next, *start))
            } else if end == &check {
                Some((pos_next, *end))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn get_loop(input: &str) -> Vec<(usize, usize)> {
    let mut map: Vec<Vec<Item>> = input
        .lines()
        .map(|line| line.chars().map(|symbol| Item::from(symbol)).collect())
        .collect();

    let mut start = (0, 0);

    for (row, x) in map.iter().enumerate() {
        for (col, item) in x.iter().enumerate() {
            match &item {
                Item::Start => start = (row, col),
                _ => continue,
            };
        }
    }

    let candidates = "|-LJ7F";
    let mut the_loop: Vec<(usize, usize)> = vec![];
    for c in candidates.chars() {
        map[start.0][start.1] = c.into();
        let mut pos = start;
        let mut from = Connection::East;
        let mut cand_loop = vec![];

        while let Some((next_pos, next_conn)) = next(&map, pos, &from) {
            pos = next_pos;
            from = next_conn;

            cand_loop.push(pos);
            if next_pos == start {
                the_loop = cand_loop;
                break;
            }
        }
    }

    the_loop
}

fn part_one(input: &str) -> usize {
    get_loop(input).len() / 2
}

fn part_two(input: &str) -> i32 {
    let loop_positions = get_loop(input);
    let count = loop_positions.len();

    let area: i32 = (0..count)
        .map(|i| {
            let xi = loop_positions[i].0;
            let yi = loop_positions[i].1;
            let xi1 = loop_positions[(i + 1) % count].0;
            let yi1 = loop_positions[(i + 1) % count].1;
            (xi * yi1) as i32 - (yi * xi1) as i32
        })
        .sum::<i32>()
        .abs()
        / 2;

    area + 1 - loop_positions.len() as i32 / 2
}
