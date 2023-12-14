use std::{env,fs,process};
use std::collections::HashSet;

fn run1(input: &str) -> u32 {
    let array: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut res = 0;
    for j in 0..array[0].len() {
        let mut load = array.len() as u32;
        for i in 0..array.len() {
            match array[i][j] {
                'O' => {
                    res += load;
                    load -= 1;
                },
                '.' => {},
                '#' => {
                    load = (array.len() - i - 1) as u32;
                },
                _ => unreachable!(),
            }
        }
    }
    res
}

fn get_load(rounds: &HashSet<(usize, usize)>, height: usize) -> usize {
    let mut load = 0;
    for (i,_) in rounds {
        load += height - i;
    }
    load
}

fn tilt_north(rounds: &HashSet<(usize, usize)>, squares: &HashSet<(usize, usize)>, height: usize, width: usize) -> HashSet<(usize, usize)> {
    let mut new_rounds = HashSet::new();
    for j in 0..width {
        let mut next_empty = 0;
        for i in 0..height {
            if squares.contains(&(i,j)) {
                next_empty = i + 1;
            } else if rounds.contains(&(i,j)) {
                new_rounds.insert((next_empty, j));
                next_empty += 1;
            }
        }
    }
    new_rounds
}

fn tilt_south(rounds: &HashSet<(usize, usize)>, squares: &HashSet<(usize, usize)>, height: usize, width: usize) -> HashSet<(usize, usize)> {
    let mut new_rounds = HashSet::new();
    for j in 0..width {
        let mut next_empty = height - 1;
        for i in (0..height).rev() {
            if squares.contains(&(i,j)) && i > 0 {
                next_empty = i - 1;
            } else if rounds.contains(&(i,j)) {
                new_rounds.insert((next_empty, j));
                if next_empty > 0 {
                    next_empty -= 1;
                }
            }
        }
    }
    new_rounds
}

fn tilt_west(rounds: &HashSet<(usize, usize)>, squares: &HashSet<(usize, usize)>, height: usize, width: usize) -> HashSet<(usize, usize)> {
    let mut new_rounds = HashSet::new();
    for i in 0..height {
        let mut next_empty = 0;
        for j in 0..width {
            if squares.contains(&(i,j)) {
                next_empty = j + 1;
            } else if rounds.contains(&(i,j)) {
                new_rounds.insert((i, next_empty));
                next_empty += 1;
            }
        }
    }
    new_rounds
}

fn tilt_east(rounds: &HashSet<(usize, usize)>, squares: &HashSet<(usize, usize)>, height: usize, width: usize) -> HashSet<(usize, usize)> {
    let mut new_rounds = HashSet::new();
    for i in 0..height {
        let mut next_empty = width - 1;
        for j in (0..width).rev() {
            if squares.contains(&(i,j)) && j > 0 {
                next_empty = j - 1;
            } else if rounds.contains(&(i,j)) {
                new_rounds.insert((i, next_empty));
                if next_empty > 0 {
                    next_empty -= 1;
                }
            }
        }
    }
    new_rounds
}

fn cycle(rounds: &HashSet<(usize, usize)>, squares: &HashSet<(usize, usize)>, height: usize, width: usize) -> HashSet<(usize, usize)> {
    let mut temp = tilt_north(&rounds, &squares, height, width);
    temp = tilt_west(&temp, &squares, height, width);
    temp = tilt_south(&temp, &squares, height, width);
    temp = tilt_east(&temp, &squares, height, width);
    temp
}

fn run2(input: &str) -> usize {
    let mut rounds = HashSet::new();
    let mut squares = HashSet::new();
    let mut i = 0;
    let mut j = 0;
    for line in input.lines() {
        j = 0;
        for c in line.chars() {
            match c {
                'O' => {
                    rounds.insert((i,j));
                },
                '#' => {
                    squares.insert((i,j));
                },
                '.' => {},
                _ => unreachable!(),
            }
            j += 1;
        }
        i += 1;
    }
    let height = i;
    let width = j;
    let mut past_cycles = vec![rounds.clone()];
    loop {
        rounds = cycle(&rounds, &squares, height, width);
        if let Some(idx) = past_cycles.iter().position(|r| *r == rounds) {
            let cycle_len = past_cycles.len() - idx;
            let final_idx = idx + (1_000_000_000 - idx) % cycle_len;
            return get_load(&past_cycles[final_idx], height);
        }
        past_cycles.push(rounds.clone());
    }
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
    assert_eq!(res, 136);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 108955);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 64);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 106689);
}
