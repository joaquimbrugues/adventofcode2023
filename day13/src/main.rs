use std::{env,fs,process};

fn run1(input: &str) -> u32 {
    let mut res = 0;
    for block in input.split("\n\n") {
        let layout: Vec<Vec<bool>> = block.lines().map(|line| {
            line.chars().map(|c| {
                match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                }
            }).collect()
        }).collect();

        for s in 1..layout.len() {
            let mut i = 0;
            let mut mirrors = true;
            while mirrors && s + i < layout.len() && i < s {
                let mut j = 0;
                while mirrors && j < layout[s].len() {
                    mirrors &= layout[s-i-1][j] == layout[s+i][j];
                    j += 1;
                }
                i += 1;
            }
            if mirrors {
                res += 100 * s as u32;
                break;
            }
        }

        for s in 1..layout[0].len() {
            let mut j = 0;
            let mut mirrors = true;
            while mirrors && s + j < layout[0].len() && j < s {
                let mut i = 0;
                while mirrors && i < layout.len() {
                    mirrors &= layout[i][s-j-1] == layout[i][s+j];
                    i += 1;
                }
                j += 1;
            }
            if mirrors {
                res += s as u32;
                break
            }
        }
    }

    res
}

fn run2(input: &str) -> u32 {
    let mut res = 0;
    for block in input.split("\n\n") {
        let layout: Vec<Vec<bool>> = block.lines().map(|line| {
            line.chars().map(|c| {
                match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                }
            }).collect()
        }).collect();

        for s in 1..layout.len() {
            let mut i = 0;
            let mut errors = 0;
            while errors < 2 && s + i < layout.len() && i < s {
                let mut j = 0;
                while errors < 2 && j < layout[s].len() {
                    if layout[s-i-1][j] != layout[s+i][j] { 
                        errors += 1
                    };
                    j += 1;
                }
                i += 1;
            }
            if errors == 1 {
                res += 100 * s as u32;
                break;
            }
        }

        for s in 1..layout[0].len() {
            let mut j = 0;
            let mut errors = 0;
            while errors < 2 && s + j < layout[0].len() && j < s {
                let mut i = 0;
                while errors < 2 && i < layout.len() {
                    if layout[i][s-j-1] != layout[i][s+j] {
                        errors += 1;
                    }
                    i += 1;
                }
                j += 1;
            }
            if errors == 1 {
                res += s as u32;
                break
            }
        }
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
    assert_eq!(res, 405);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 32371);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 400);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
