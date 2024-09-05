use std::{env,fs,process};
use std::collections::HashSet;

fn run1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (_, line) = line.split_once(':').unwrap();
        let (swinning, snums) = line.split_once('|').unwrap();
        let winning: HashSet<u32> = swinning.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let winners = snums.trim().split_whitespace().map(|s| s.parse().unwrap()).filter(|n| winning.contains(&n)).count() as u32;
        if winners > 0 {
            sum += 2u32.pow(winners - 1);
        }
    }
    sum
}

fn run2(input: &str) -> u32 {
    let mut scratch = Vec::with_capacity(input.lines().count());
    for _ in 0..input.lines().count() {
        scratch.push(1);
    }
    let mut i = 0;
    for line in input.lines() {
        let (_, line) = line.split_once(':').unwrap();
        let (swinning, snums) = line.split_once('|').unwrap();
        let winning: HashSet<u32> = swinning.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let mut winners = snums.trim().split_whitespace().map(|s| s.parse().unwrap()).filter(|n| winning.contains(&n)).count() as u32;
        let mut j = i + 1;
        while j < scratch.len() && winners > 0 {
            scratch[j] += scratch[i];
            j += 1;
            winners -= 1;
        }
        i += 1;
    }
    scratch.iter().sum()
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
    assert_eq!(res, 13);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 21821);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 30);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,5539496);
}
