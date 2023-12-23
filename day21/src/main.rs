use std::{env,fs,process};
use std::collections::{VecDeque,HashSet};

fn neighbours(pt: (isize, isize)) -> [(isize, isize); 4] {
    [
        (pt.0 - 1, pt.1),
        (pt.0 + 1, pt.1),
        (pt.0, pt.1 - 1),
        (pt.0, pt.1 + 1),
    ]
}

fn run1(input: &str, steps: usize) -> usize {
    let mut map = HashSet::new();
    let mut start = None;

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let i = i as isize;
            let j = j as isize;
            match c {
                '#' => {},
                '.' => { map.insert((i,j)); },
                'S' => {
                    map.insert((i,j));
                    start = Some((i,j));
                },
                _ => unreachable!(),
            }
        }
    }

    let start = start.unwrap();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut reachable = HashSet::new();
    let mut visited = HashSet::new();

    while let Some((pos, dist)) = queue.pop_front() {
        if dist == steps {
            reachable.insert(pos);
        } else if !visited.contains(&(pos, dist)) {
            visited.insert((pos, dist));
            for n in neighbours(pos) {
                if map.contains(&n) {
                    queue.push_back((n, dist + 1));
                }
            }
        }
    }
    reachable.len()
}

fn cong(pt: (isize, isize), size: (isize, isize)) -> (isize, isize) {
    (pt.0 % size.0, pt.1 % size.1)
}

fn run2(input: &str, steps: u64) -> usize {
    let mut map = HashSet::new();
    let mut start = None;
    let mut height = 0;
    let mut width = 0;

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let i = i as isize;
            let j = j as isize;
            match c {
                '#' => {},
                '.' => { map.insert((i,j)); },
                'S' => {
                    map.insert((i,j));
                    start = Some((i,j));
                },
                _ => unreachable!(),
            }
            if j > width {
                width = j;
            }
        }
        if i > height {
            height = i;
        }
    }
    let height = (height + 1) as isize;
    let width = (width + 1) as isize;
    let size = (height, width);

    let start = start.unwrap();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut reachable = HashSet::new();
    let mut visited = HashSet::new();

    while let Some((pos, dist)) = queue.pop_front() {
        println!("{pos:?}, steps: {dist}");
        if dist == steps {
            reachable.insert(pos);
        } else if !visited.contains(&(pos, dist)) {
            visited.insert((pos, dist));
            for n in neighbours(pos) {
                if map.contains(&(cong(n, size))) {
                    queue.push_back((n, dist + 1));
                }
            }
        }
    }
    println!("{reachable:?}");
    reachable.len()
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

    let res = run2(&input, 10);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input, 6);
    assert_eq!(res, 16);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input, 64);
    assert_eq!(res, 3677);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input, 6);
    assert_eq!(res, 16);
    let res = run2(&input, 10);
    assert_eq!(res, 50);
    let res = run2(&input, 50);
    assert_eq!(res, 1594);
    let res = run2(&input, 100);
    assert_eq!(res, 6536);
    let res = run2(&input, 500);
    assert_eq!(res, 167004);
    let res = run2(&input, 1000);
    assert_eq!(res, 668697);
    let res = run2(&input, 5000);
    assert_eq!(res, 16733044);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
