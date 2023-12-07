use std::{env,fs,process};
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq,)]
enum HandType { Five, Four, House, Three, Pairs, Pair, High, }

impl From<HandType> for u8 {
    fn from(handtype: HandType) -> Self {
        use HandType::*;
        match handtype {
            High => 0,
            Pair => 1,
            Pairs => 2,
            Three => 3,
            House => 4,
            Four => 5,
            Five => 6,
        }
    }
}

impl From<[u8;5]> for HandType {
    fn from(hand: [u8;5]) -> Self {
        use HandType::*;
        let mut summary = HashMap::new();
        let mut jokers = 0;
        for c in hand {
            if c == 1 {
                jokers += 1;
            } else if let Some(n) = summary.get_mut(&c) {
                *n += 1;
            } else {
                summary.insert(c,1);
            }
        }
        // If the hand is all jokers, just take all equal to A
        if jokers == 5 {
            return Five;
        } else if jokers > 0 {
            // Transform all jokers into the most frequent card
            *summary.values_mut().max().unwrap() += jokers;
        }
        match summary.len() {
            1 => Five,
            4 => Pair,
            5 => High,
            2 => {
                // Either Four of a kind or Full House
                if summary.values().find(|&&n| n == 4).is_some() {
                    Four
                } else if summary.values().find(|&&n| n == 3).is_some() {
                    House
                } else {
                    unreachable!()
                }
            },
            3 => {
                // Either two Pairs or Three of a kind
                if summary.values().find(|&&n| n == 3).is_some() {
                    Three
                } else if summary.values().filter(|&&n| n == 2).count() == 2 {
                    Pairs
                } else {
                    unreachable!()
                }
            },
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq,)]
struct Hand {
    hand: [u8;5],
    kind: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let t1: u8 = self.kind.into();
        let t2: u8 = other.kind.into();
        if t1 == t2 {
            let mut i = 0;
            while self.hand[i] == other.hand[i] { i += 1; }
            self.hand[i].cmp(&other.hand[i])
        } else {
            t1.cmp(&t2)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn insert_ordered(vec: &mut Vec<(Hand, u32)>, (hand, bid): (Hand, u32)) -> usize {
    let mut i = 0;
    while i < vec.len() && vec[i].0 < hand {
        i += 1;
    }
    vec.insert(i,(hand, bid));
    i + 1
}

// Code for cards:
// 2-9 - Themselves
// T = 10, J = 11, Q = 12, K = 13, A = 14

// Part 2: J = 1

fn run(input: &str, jokers: bool) -> u32 {
    let mut list = vec![];
    for line in input.lines() {
        let (first, second) = line.split_once(' ').unwrap();
        let cs = first.chars().collect::<Vec<_>>();
        assert_eq!(cs.len(), 5);
        let mut array = [0u8;5];
        for i in 0..5 {
            array[i] = if cs[i].is_digit(10) {
                cs[i].to_digit(10).unwrap() as u8
            } else {
                match cs[i] {
                    'T' => 10,
                    'J' => {
                        if jokers {
                            1
                        } else {
                            11
                        }
                    },
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => unreachable!(),
                }
            };
        }
        let bid: u32 = second.parse().unwrap();
        let hand = Hand { hand: array, kind: HandType::from(array), };
        insert_ordered(&mut list, (hand, bid));
    }
    list.iter().enumerate().fold(0, |acc, (i, (_, bid))| {
        let i = (i + 1) as u32;
        acc + i * bid
    })
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

    let res = run(&input, true);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run(&input, false);
    assert_eq!(res, 6440);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run(&input, false);
    assert_eq!(res, 247961593);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run(&input, true);
    assert_eq!(res, 5905);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run(&input, true);
    assert_eq!(res, 248750699);
}
