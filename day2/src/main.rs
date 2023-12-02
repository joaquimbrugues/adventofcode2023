use std::{env,fs,process};

const BOUNDS : [(&str, u32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

fn run1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (head, tail) = line.split_once(':').unwrap();
        let id : u32 = head.split_once(' ').unwrap().1.parse().unwrap();
        let mut valid = true;
        for game in tail.split(';').map(|s| s.trim()) {
            let mut iterator = game.split(',').map(|s| s.trim());
            while let Some(play) = iterator.next() {
                let num: u32 = play.split_once(' ').unwrap().0.parse().unwrap();
                let color = play.split_once(' ').unwrap().1;
                for (col, bound) in BOUNDS {
                    if col == color {
                        valid &= num <= bound;
                    }
                }
            }
            if !valid { break; }
        }
        if valid {
            sum += id;
        }
    }
    sum
}

fn run2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut rgb = [0, 0, 0];
        let (_, tail) = line.split_once(':').unwrap();
        for game in tail.split(';').map(|s| s.trim()) {
            for play in game.split(',').map(|s| s.trim()) {
                let num: u32 = play.split_once(' ').unwrap().0.parse().unwrap();
                let color = match play.split_once(' ').unwrap().1 {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => panic!("Unrecognized color!"),
                };
                if num > rgb[color] {
                    rgb[color] = num;
                }
            }
        }
        sum += rgb[0] * rgb[1] * rgb[2];
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
    assert_eq!(res, 2795);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 2286);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 75561);
}
