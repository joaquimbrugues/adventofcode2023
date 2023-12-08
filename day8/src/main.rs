use std::{env,fs,process};
use std::collections::HashMap;

fn run1(input: &str) -> u32 {
    let (head, tail) = input.split_once("\n\n").unwrap();
    let tree: HashMap<&str, (&str, &str)> = tail.lines().map(|line| {
        let (mut head, tail) = line.split_once('=').unwrap();
        head = head.trim();
        let (mut left, mut right) = tail.split_once(", ").unwrap();
        left = left.trim().strip_prefix('(').unwrap();
        right = right.trim().strip_suffix(')').unwrap();
        (head, (left, right))
    }).collect();

    let mut steps = 0;
    let mut current_node = "AAA";
    let mut map = head.trim().chars().cycle();
    while current_node != "ZZZ" {
        let c = map.next().unwrap();
        current_node = match c {
            'L' => tree.get(current_node).unwrap().0,
            'R' => tree.get(current_node).unwrap().1,
            _ => unreachable!(),
        };
        steps += 1;
    }
    steps
}

fn mcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        mcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / mcd(a,b)
}

fn run2(input: &str) -> u64 {
    let (head, tail) = input.split_once("\n\n").unwrap();
    let head = head.trim();
    let tree: HashMap<&str, (&str, &str)> = tail.lines().map(|line| {
        let (mut head, tail) = line.split_once('=').unwrap();
        head = head.trim();
        let (mut left, mut right) = tail.split_once(", ").unwrap();
        left = left.trim().strip_prefix('(').unwrap();
        right = right.trim().strip_suffix(')').unwrap();
        (head, (left, right))
    }).collect();
    let init_nodes: Vec<&str> = tail.lines().map(|line| line.split_once('=').unwrap().0.trim()).filter(|s| s.ends_with('A')).collect();

    let mut acc = 1;
    for n in init_nodes {
        let mut steps = 0;
        let mut map = head.chars().cycle();
        let mut current_node = n;
        while !current_node.ends_with('Z') {
            let c = map.next().unwrap();
            current_node = match c {
                'L' => tree.get(current_node).unwrap().0,
                'R' => tree.get(current_node).unwrap().1,
                _ => unreachable!(),
            };
            steps += 1;
        }
        acc = lcm(acc, steps);
    }
    acc
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
fn example11() {
    let input = fs::read_to_string("test1.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 2);
}

#[test]
fn example12() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 6);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 18023);
}

#[test]
fn example23() {
    let input = fs::read_to_string("test3.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 6);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,14449445933179);
}
