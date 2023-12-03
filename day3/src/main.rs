use std::{env,fs,process};
use std::collections::HashSet;

fn neighbours((i, j): (isize, isize)) -> [(isize, isize); 8] {
    [
        (i-1, j-1),
        (i-1, j),
        (i-1, j+1),
        (i, j-1),
        (i, j+1),
        (i+1, j-1),
        (i+1, j),
        (i+1, j+1),
    ]
}

fn run1(input: &str) -> u32 {
    // First, mark positions where a part number will be considered
    let mut parts = HashSet::new();
    let mut i = 0;
    let mut j;
    for line in input.lines().map(|s| s.trim()) {
        j = 0;
        for c in line.chars() {
            if !(c.is_digit(10) || c == '.') {
                for p in neighbours((i,j)) {
                    parts.insert(p);
                }
            }
            j += 1;
        }
        i += 1;
    }
    
    // Now parse the numbers and add them if they belong to parts in some digit
    i = 0;
    let mut sum = 0;
    let mut current;
    let mut is_part;
    for line in input.lines().map(|s| s.trim()) {
        j = 0;
        current = 0;
        is_part = false;
        for c in line.chars() {
            if c.is_digit(10) {
                is_part |= parts.contains(&(i,j));
                current *= 10;
                current += c.to_digit(10).unwrap();
            } else {
                // End parsing number
                if is_part {
                    //println!("{current}");
                    sum += current;
                }
                current = 0;
                is_part = false;
            }
            j += 1;
        }
        // If a number is partially parsed, add it
        if is_part {
            //println!("{current}");
            sum += current;
        }
        i += 1;
    }
    sum
}

fn run2(input: &str) -> u32 {
    // First parse all the numbers and note their positions
    let mut numbers = vec![];
    let mut current_positions;
    let mut current_num;
    let mut i = 0;
    let mut j;
    for line in input.lines().map(|s| s.trim()) {
        j = 0;
        current_positions = HashSet::new();
        current_num = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                current_positions.insert((i,j));
                current_num *= 10;
                current_num += c.to_digit(10).unwrap();
            } else {
                if !current_positions.is_empty() && current_num > 0 {
                    numbers.push((current_num, current_positions.clone()));
                    current_num = 0;
                    current_positions = HashSet::new();
                }
            }
            j += 1;
        }
        if !current_positions.is_empty() && current_num > 0 {
            numbers.push((current_num, current_positions.clone()));
        }
        i += 1;
    }

    // Second, look for all the '*', see if they have exactly two neighbouring numbers, and
    // multiply + add them
    let mut sum = 0;
    i = 0;
    for line in input.lines().map(|s| s.trim()) {
        j = 0;
        for c in line.chars() {
            if c == '*' {
                let neighs = neighbours((i,j));
                let nums: Vec<&u32> = numbers.iter().filter(|(_, h)| {
                    neighs.iter().fold(false, |acc, n| { acc || h.contains(&n) })
                }).map(|(n,_)| n).collect();
                if nums.len() == 2 {
                    sum += nums[0] * nums[1];
                }
            }
            j += 1;
        }
        i += 1;
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
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 4361);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 539713);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 467835);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 84159075);
}
