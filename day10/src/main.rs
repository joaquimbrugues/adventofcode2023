use std::{env,fs,process};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
enum Direction { North, East, South, West, }

impl Direction {
    fn parse(c: char) -> Option<(Self, Self)> {
        use Direction::*;
        match c {
            '|' => Some((North, South)),
            '-' => Some((East, West)),
            'L' => Some((North, East)),
            'J' => Some((North, West)),
            '7' => Some((South, West)),
            'F' => Some((East, South)),
            '.' | 'S' => None,
            _ => unreachable!(),
        }
    }

    fn all() -> [Self; 4] {
        use Direction::*;
        [North, East, South, West]
    }

    fn opp(&self) -> Self {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

fn add(start: (i32, i32), dir: Direction) -> (i32, i32) {
    use Direction::*;
    match dir {
        North => (start.0 - 1, start.1),
        East => (start.0, start.1 + 1),
        South => (start.0 + 1, start.1),
        West => (start.0, start.1 - 1),
    }
}

fn run1(input: &str) -> u32 {
    // Parse the input
    // Pipes is a map (pos) -> (Dir, Dir)
    let mut pipes = HashMap::new();
    let mut i = 0;
    let mut starting = None;
    for line in input.lines() {
        let mut j = 0;
        for c in line.trim().chars() {
            if c == 'S' {
                if starting.is_none() {
                    starting = Some((i,j));
                } else {
                    panic!("Starting assigned twice!");
                }
            } else {
                if let Some(dirs) = Direction::parse(c) {
                    pipes.insert((i,j), dirs);
                }
            }
            j += 1;
        }
        i += 1;
    }

    let starting = starting.unwrap();
    // Depth-first search
    // Find a path back to the starting node, registering distance
    // Stack contains node to visit, direction we are arribing from, and distance
    let mut stack = vec![];
    for dir in Direction::all() {
        stack.push((add(starting, dir), dir.opp(), 1));
    }
    //let mut visited = HashSet::new();
    while let Some((pos, dir_from, dist)) = stack.pop() {
        if pos == starting {
            return dist / 2 + (dist % 2);
        } else {
            // Make sure that pipes directions match before anything else
            if let Some(&(dir1, dir2)) = pipes.get(&pos) {
                let dir_to = if dir_from == dir1 {
                    dir2
                } else if dir_from == dir2 {
                    dir1
                } else {
                    continue;
                };
                stack.push((add(pos, dir_to), dir_to.opp(), dist+1));
            }
        }
    }
    unreachable!()
}

fn run2(input: &str) -> u32 {
    0
}

fn main() {
    let mut args = env::args();
    let filepath;
    args.next();
    if let Some(s) = args.next() {
        filepath = s;
    }
    else {
        eprintln!("Give me a file name! I must feeds on files! Aaargh!");
        process::exit(1);
    }

    let input = fs::read_to_string(filepath).unwrap();

    let res = run1(&input);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 8);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 6831);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 10);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
