use std::{env,fs,process};

fn run1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += line.chars().filter(|c| c.is_digit(10)).nth(0).unwrap().to_digit(10).unwrap() * 10;
        sum += line.chars().filter(|c| c.is_digit(10)).last().unwrap().to_digit(10).unwrap();
    }
    sum
}

const DIGITS: [(&str, u32); 9] = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

fn find_first(line: &str) -> u32 {
    let mut dig = (1000, 0);
    if let Some((i, d)) = line.chars().enumerate().filter(|(_, c)| c.is_digit(10)).map(|(i,c)| (i, c.to_digit(10).unwrap())).nth(0) {
        dig = (i, d);
    }
    for (s, u) in DIGITS {
        if let Some((i, _)) = line.match_indices(s).nth(0) {
            if i < dig.0 {
                dig = (i, u);
            }
        }
    }
    dig.1
}

fn find_last(line: &str) -> u32 {
   let mut dig = (0, 0);
   if let Some((i, d)) = line.chars().enumerate().filter(|(_, c)| c.is_digit(10)).map(|(i,c)| (i, c.to_digit(10).unwrap())).last() {
       dig = (i, d);
   }
   for (s, u) in DIGITS {
       if let Some((i, _)) = line.match_indices(s).last() {
           if i > dig.0 {
               dig = (i, u);
           }
       }
   }
   dig.1
}

fn run2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += find_first(line) * 10;
        sum += find_last(line);
    }
    sum
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
    let input = fs::read_to_string("test1.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 142);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 54573);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 281);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 54591);
}
