use std::{env,fs,process};
use std::collections::{HashSet, HashMap,};

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

fn run1(input: &str) -> usize {
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
    let (cycle, _, _) = get_loop(starting, &pipes);
    let l = cycle.len();
    l / 2 + (l % 2)
}

// Return the loop + the two directions required for the starting node to close it
fn get_loop(starting: (i32, i32), pipes: &HashMap<(i32,i32), (Direction, Direction)>) -> (HashSet<(i32,i32)>, Direction, Direction) {
    for dir in Direction::all() {
        let mut cycle = HashSet::from([starting]);
        let mut next = (add(starting, dir), dir.opp());
        while let Some(&(dir1,dir2)) = pipes.get(&next.0) {
            if next.1 == dir1 {
                cycle.insert(next.0);
                next = (add(next.0, dir2), dir2.opp());
            } else if next.1 == dir2 {
                cycle.insert(next.0);
                next = (add(next.0, dir1), dir1.opp());
            } else {
                // Pipes do not match
                break;
            }
        }
        if next.0 == starting {
            return (cycle, dir, next.1);
        }
    }
    panic!("No direction closes a loop!")
}

fn run2(input: &str) -> u32 {
    let input = if let Some((head, _)) = input.split_once("\n\n") {
        head
    } else {
        input
    };
    // Parse the input
    // Pipes is a map (pos) -> (Dir, Dir)
    let mut pipes = HashMap::new();
    let mut i = 0;
    let mut j = 0;
    let mut starting = None;
    for line in input.lines() {
        j = 0;
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
    let (cycle, dir1, dir2) = get_loop(starting, &pipes);
    pipes.insert(starting, (dir1, dir2));
    
    // Scan lines
    let mut inner = 0;
    for k in 0..i {
        let mut inside = false;
        let mut last_corner: Option<Direction> = None;
        for l in 0..j {
            if cycle.contains(&(k,l)) {
                use Direction::*;
                match pipes.get(&(k,l)).unwrap() {
                    (North, South) | (South, North) => {
                        // Vertical pipe
                        inside = !inside;
                    },
                    (West, East) | (East, West) => {
                        // Horizontal pipe
                        // Do nothing
                    },
                    (East, North) | (North, East) => {
                        // Corner facing east.
                        // last_corner should be None, and we change to vertical direction
                        assert!(last_corner.is_none());
                        last_corner = Some(North);
                    },
                    (East, South) | (South, East) => {
                        // Corner facing east.
                        // last_corner should be None, and we change to vertical direction
                        assert!(last_corner.is_none());
                        last_corner = Some(South);
                    },
                    (West, North) | (North, West) => {
                        // Corner facing west.
                        // last_corner should be Some, and we change inside/outside depending on it
                        if let Some(d) = last_corner {
                            if d == South {
                                inside = !inside;
                            }
                        } else {
                            panic!("Unreachable corner!")
                        }
                        last_corner = None;
                    },
                    (West, South) | (South, West) => {
                        // Corner facing west.
                        // last_corner should be Some, and we change inside/outside depending on it
                        if let Some(d) = last_corner {
                            if d == North {
                                inside = !inside;
                            }
                        } else {
                            panic!("Unreachable corner!")
                        }
                        last_corner = None;
                    },
                    _ => unreachable!(),
                }
            } else if inside {
                inner += 1;
            }
        }
    }
    inner
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

    let res = run2(&input);
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

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 305);
}
