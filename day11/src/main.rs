use std::{env,fs,process};
use std::collections::HashSet;

fn run(input: &str, factor: usize) -> usize {
    let mut i = 0;
    let mut j;
    let mut galaxies = vec![];
    let mut rows = HashSet::new();
    let mut columns = HashSet::new();
    for line in input.lines() {
        j = 0;
        for c in line.chars() {
            match c {
                '#' => {
                    galaxies.push((i,j));
                    rows.insert(i);
                    columns.insert(j);
                },
                '.' => {},
                _ => unreachable!(),
            }
            j += 1;
        }
        i += 1;
    }

    let mut res = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            let (y1,y2) = if galaxies[i].0 < galaxies[j].0 {
                (galaxies[i].0, galaxies[j].0)
            } else {
                (galaxies[j].0, galaxies[i].0)
            };
            let (x1,x2) = if galaxies[i].1 < galaxies[j].1 {
                (galaxies[i].1, galaxies[j].1)
            } else {
                (galaxies[j].1, galaxies[i].1)
            };
            res += (y2 - y1 + ((y1..y2).filter(|y| !rows.contains(y)).count() * (factor - 1))) + (x2 - x1 + ((x1..x2).filter(|x| !columns.contains(x)).count() * (factor - 1)));
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

    let res = run(&input, 1000000);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run(&input, 2);
    assert_eq!(res, 374);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run(&input, 2);
    assert_eq!(res, 9623138);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run(&input, 10);
    assert_eq!(res, 1030);
    let res = run(&input, 100);
    assert_eq!(res, 8410);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run(&input, 1000000);
    assert_eq!(res, 726820169514);
}
