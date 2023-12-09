use std::{env,fs,process};

fn run1(input: &str) -> i32 {
    let mut res = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let mut triangle = vec![nums];

        while triangle.last().unwrap().iter().find(|&&n| n != 0).is_some() {
            let tip = triangle.last().unwrap();
            triangle.push(tip.iter().skip(1).zip(tip.iter()).map(|(a,b)| a - b).collect());
        }
        triangle.last_mut().unwrap().push(0);
        res += triangle.iter().map(|v| v.last().unwrap()).sum::<i32>();
    }
    res
}

fn run2(input: &str) -> i32 {
    let mut res = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let mut triangle = vec![nums];

        while triangle.last().unwrap().iter().find(|&&n| n != 0).is_some() {
            let tip = triangle.last().unwrap();
            triangle.push(tip.iter().skip(1).zip(tip.iter()).map(|(a,b)| a - b).collect());
        }
        res += triangle.iter().map(|v| v[0]).enumerate().map(|(i,n)| {
            if i % 2 == 0 {
                n
            } else {
                - n
            }
        }).sum::<i32>();
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
    assert_eq!(res, 114);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 1479011877);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 2);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 973);
}
