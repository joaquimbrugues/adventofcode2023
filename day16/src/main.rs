use std::{env,fs,process};
use std::collections::{HashSet,VecDeque};

#[derive(PartialEq,Eq,Clone,Copy,Debug,Hash,)]
enum Dir { Up, Down, Left, Right, }

impl Dir {
    fn interact(&self, c: char) -> Vec<Self> {
        use Dir::*;
        match c {
            '.' => vec![*self],
            '|' => {
                if *self == Up || *self == Down {
                    vec![*self]
                } else {
                    vec![Up, Down]
                }
            },
            '-' => {
                if *self == Left || *self == Right {
                    vec![*self]
                } else {
                    vec![Left, Right]
                }
            },
            '\\' => {
                match self {
                    Up => vec![Left],
                    Down => vec![Right],
                    Left => vec![Up],
                    Right => vec![Down],
                }
            },
            '/' => {
                match self {
                    Up => vec![Right],
                    Down => vec![Left],
                    Left => vec![Down],
                    Right => vec![Up],
                }
            },
            _ => unreachable!(),
        }
    }
}

fn walk_map(array: &Vec<Vec<char>>, init: (usize, usize, Dir)) -> usize {
    let mut queue = VecDeque::from([init]);
    let mut explored = HashSet::new();
    explored.insert(queue[0]);
    while let Some((i,j,dir)) = queue.pop_front() {
        for d in dir.interact(array[i][j]) {
            let next = match d {
                Dir::Up => {
                    if i > 0 {
                        Some((i-1,j))
                    } else {
                        None
                    }
                },
                Dir::Down => {
                    if i + 1 < array.len() {
                        Some((i+1, j))
                    } else {
                        None
                    }
                },
                Dir::Left => {
                    if j > 0 {
                        Some((i, j-1))
                    } else {
                        None
                    }
                },
                Dir::Right => {
                    if j + 1 < array[i].len() {
                        Some((i, j+1))
                    } else {
                        None
                    }
                },
            };

            if let Some((ni,nj)) = next {
                if !explored.contains(&(ni,nj,d)) {
                    explored.insert((ni,nj,d));
                    queue.push_back((ni,nj,d));
                }
            }
        }
    }

    let explored: HashSet<_> = explored.into_iter().map(|(i,j,_)| (i,j)).collect();
    explored.len()
}

fn run1(input: &str) -> usize {
    let array: Vec<Vec<_>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();
    walk_map(&array, (0,0,Dir::Right))
}

fn run2(input: &str) -> usize {
    let array: Vec<Vec<_>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();
    let mut res = 0;
    // Top row
    for j in 0..array[0].len() {
        let r = walk_map(&array, (0, j, Dir::Down));
        if r > res {
            res = r;
        }
    }
    // Bottom row
    for j in 0..array[array.len()-1].len() {
        let r = walk_map(&array, (array.len()-1, j, Dir::Up));
        if r > res {
            res = r;
        }
    }
    // Left column
    for i in 0..array.len() {
        let r = walk_map(&array, (i, 0, Dir::Right));
        if r > res {
            res = r;
        }
    }
    // Right column
    for i in 0..array.len() {
        let r = walk_map(&array, (i, array[i].len() - 1, Dir::Left));
        if r > res {
            res = r;
        }
    }
    res
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
    assert_eq!(res, 46);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 7111);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 51);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 7831);
}
