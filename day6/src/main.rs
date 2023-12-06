use std::{env,fs,process};

fn solve(t: f32, r: f32) -> (f32, f32) {
    let tt = t/2.0;
    let disc = f32::sqrt(tt*tt - r);
    ( tt - disc, tt + disc )
}

fn run1(input: &str) -> f32 {
    let mut res = 1.0;
    for (t,r) in input.lines().nth(0).unwrap().split_once(':').unwrap().1.trim().split_whitespace().map(|s| s.parse::<f32>().unwrap()).zip(input.lines().nth(1).unwrap().split_once(':').unwrap().1.trim().split_whitespace().map(|s| s.parse::<f32>().unwrap())) {
        let (a, b) = solve(t,r);
        let m = f32::ceil(b - 1.0) - f32::floor(a);
        println!("{m}");
        res *= m;
    }
    res
}

fn run2(input: &str) -> u32 {
    0
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
    assert_eq!(res, 288);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 1159152);
}

//#[test]
//fn example2() {
    //let input = fs::read_to_string("test.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
