use std::{env,fs,process};
use std::collections::HashMap;

fn run1(input: &str) -> u64 {
    let mut res = 0;
    for line in input.lines() {
        // Parse input
        let (head, tail) = line.split_once(' ').unwrap();
        let array: Vec<u8> = head.chars().map(|c| match c {
            '.' => 0,
            '#' => 1,
            '?' => 2,
            _ => unreachable!(),
        }).collect();
        let groups: Vec<u8> = tail.split(',').map(|s| s.parse().unwrap()).collect();

        let mut lazy = LazyCountArrangements::new();
        res += lazy.num_valid_arrangements(&array, &groups, 0);
    }
    res
}

struct LazyCountArrangements {
    lazy: HashMap<(Vec<u8>, Vec<u8>, u8), u64>,
}

impl LazyCountArrangements {
    fn new() -> Self {
        Self { lazy: HashMap::new(), }
    }

    fn num_valid_arrangements(&mut self, springs: &Vec<u8>, clues: &Vec<u8>, damaged: u8) -> u64 {
        match self.lazy.get(&(springs.clone(), clues.clone(), damaged)) {
            Some(res) => *res,
            None => {
                // Recursion
                let res = if springs.len() == 0 {
                    if clues.len() == 1 && clues[0] == damaged {
                        // There is only one spring and it is damaged
                        1
                    } else if clues.len() == 0 && damaged == 0 {
                        // We exhaused the possibilities
                        1
                    } else {
                        0
                    }
                } else {
                    let new_springs = springs.clone().split_off(1);
                    let (clue, new_clues) = if clues.len() == 0 {
                        (0, vec![])
                    } else {
                        (clues[0], clues.clone().split_off(1))
                    };
                    match springs[0] {
                        0 => {
                            if damaged == 0 {
                                self.num_valid_arrangements(&new_springs, clues, 0)
                            } else if damaged == clue {
                                self.num_valid_arrangements(&new_springs, &new_clues, 0)
                            } else {
                                0
                            }
                        },
                        1 => {
                            if damaged > clue {
                                0
                            } else {
                                self.num_valid_arrangements(&new_springs, clues, damaged + 1)
                            }
                        },
                        2 => {
                            let mut o1 = vec![0];
                            o1.append(&mut new_springs.clone());
                            let mut o2 = vec![1];
                            o2.append(&mut new_springs.clone());
                            self.num_valid_arrangements(&o1, clues, damaged) + self.num_valid_arrangements(&o2, clues, damaged)
                        },
                        _ => unreachable!(),
                    }
                };
                self.lazy.insert((springs.clone(), clues.clone(), damaged), res);
                res
            }
        }
    }
}

fn run2(input: &str) -> u64 {
    let mut res = 0;
    for line in input.lines() {
        // Parse input
        let (head, tail) = line.split_once(' ').unwrap();
        let temp_array: Vec<u8> = head.chars().map(|c| match c {
            '.' => 0,
            '#' => 1,
            '?' => 2,
            _ => unreachable!(),
        }).collect();
        let temp_groups: Vec<u8> = tail.split(',').map(|s| s.parse().unwrap()).collect();
        let mut array = vec![];
        let mut groups = vec![];
        for i in 0..5 {
            if i > 0 {
                array.push(2);
            }
            array.append(&mut temp_array.clone());
            groups.append(&mut temp_groups.clone());
        }

        let mut lazy = LazyCountArrangements::new();
        res += lazy.num_valid_arrangements(&array, &groups, 0);

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
    assert_eq!(res, 21);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 7622);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 525152);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 4964259839627);
}
