use std::{env,fs,process};
use std::collections::HashSet;

fn run1(input: &str) -> u64 {
    let (head, tail) = input.split_once("\n\n").unwrap();
    let mut following: HashSet<u64> = head.split_once(": ").unwrap().1.split_whitespace().map(|s| s.parse().unwrap()).collect();
    for block in tail.split("\n\n") {
        let mut temp = HashSet::new();
        for line in block.lines().skip(1) {
            let parsed: Vec<u64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            assert_eq!(parsed.len(), 3);
            let (dest, source, range) = (parsed[0], parsed[1], parsed[2]);
            //for i in 0..range {
                //if following.remove(&(source + i)) {
                    //temp.insert(dest + i);
                //}
            //}
            let mut ttemp = HashSet::new();
            for v in following.iter().filter(|&&n| n >= source && n - source < range) {
                ttemp.insert(dest + v - source);
            }
            temp = temp.union(&ttemp).map(|&n| n).collect();
            following = following.difference(&ttemp.into_iter().map(|n| n + source - dest).collect()).map(|&n| n).collect();
        }
        for &v in following.iter() {
            temp.insert(v);
        }
        following = temp;
    }
    following.into_iter().min().unwrap()
}

fn run2(input: &str) -> u64 {
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
    assert_eq!(res, 35);
}

//#[test]
//fn input1() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run1(&input);
    //assert_eq!(res,42);
//}

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
