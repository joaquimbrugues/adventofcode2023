use std::{env,fs,process};

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
    assert_eq!(res, 136);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 108955);
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
