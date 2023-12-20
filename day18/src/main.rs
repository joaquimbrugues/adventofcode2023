use std::{env,fs,process};
use std::collections::HashMap;

#[derive(Debug,Clone,Copy,PartialEq,Eq,)]
enum Dir { Up, Down, Left, Right, }

impl Dir {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'U' | '3' => Some(Self::Up),
            'D' | '1' => Some(Self::Down),
            'L' | '2' => Some(Self::Left),
            'R' | '0' => Some(Self::Right),
            _ => None,
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "U" => Some(Self::Up),
            "D" => Some(Self::Down),
            "L" => Some(Self::Left),
            "R" => Some(Self::Right),
            _ => None,
        }
    }

    fn next(&self, pt: (i32,i32)) -> (i32, i32) {
        match self {
            Self::Up => (pt.0 - 1, pt.1),
            Self::Down => (pt.0 + 1, pt.1),
            Self::Left => (pt.0, pt.1 - 1),
            Self::Right => (pt.0, pt.1 + 1),
        }
    }
}

fn get_area(circ: &HashMap<(i32,i32),(Dir,Dir)>, minx: i32, maxx: i32, miny: i32, maxy: i32) -> u64 {
    use Dir::*;

    let mut res = 0;
    for i in miny..=maxy {
        let mut inside = false;
        let mut last_corner = None;
        for j in minx..=maxx {
            match circ.get(&(i,j)) {
                None => {
                    if inside {
                        res += 1;
                    }
                },
                Some(c) => {
                    res += 1;
                    match c {
                        (Up,Up) | (Down,Down) => {
                            //Vertical trench
                            inside = !inside;
                        },
                        (Left,Left) | (Right, Right) => {
                            //Horizontal trench -- Do nothing
                        },
                        (Left,Up) | (Down,Right) => {
                            // Corner of type |_, register the last direction as "Up"
                            assert!(last_corner.is_none());
                            last_corner = Some(Up);
                        },
                        (Left, Down) | (Up, Right) => {
                            // Corner of type |‾, register the last direction as "Down"
                            assert!(last_corner.is_none());
                            last_corner = Some(Down);
                        },
                        (Right, Up) | (Down, Left) => {
                            // Corner of type _|, change "inside" if the last direction was "Down"
                            if let Some(d) = last_corner {
                                if d == Down {
                                    inside = !inside;
                                }
                            } else {
                                panic!("Unreachable corner at ({i},{j})!");
                            }
                            last_corner = None;
                        },
                        (Right, Down) | (Up, Left) => {
                            // Corner of type ‾|, change "inside" if the last direction was "Up"
                            if let Some(d) = last_corner {
                                if d == Up {
                                    inside = !inside;
                                }
                            } else {
                                panic!("Unreachable corner at ({i},{j})!");
                            }
                            last_corner = None;
                        },
                        _ => unreachable!(),
                    }
                },
            }
        }
    }
    res
}

fn run1(input: &str) -> u64 {
    let mut last_dir = Dir::from_char(input.lines().last().unwrap().chars().nth(0).unwrap()).unwrap();
    let mut circ = HashMap::new();
    let mut current_pos = (0,0);
    let mut maxy = 0;
    let mut miny = 0;
    let mut maxx = 0;
    let mut minx = 0;
    for line in input.lines() {
        let v: Vec<_> = line.splitn(3, ' ').collect();
        let (d, n) = (Dir::from_str(v[0]).unwrap(), v[1].parse().unwrap());
        circ.insert(current_pos, (last_dir,d));
        for _ in 1..n {
            current_pos = d.next(current_pos);
            circ.insert(current_pos,(d,d));
        }
        current_pos = d.next(current_pos);
        last_dir = d;
        if current_pos.0 > maxy {
            maxy = current_pos.0;
        }
        if current_pos.0 < miny {
            miny = current_pos.0;
        }
        if current_pos.1 > maxx {
            maxx = current_pos.1;
        }
        if current_pos.1 < minx {
            minx = current_pos.1;
        }
    }

    get_area(&circ, minx, maxx, miny, maxy)
}

fn parse_num(input: &str) -> (Dir, i64) {
    let d = Dir::from_char(input.chars().last().unwrap()).unwrap();
    let mut n = 0;
    for c in input[0..input.len()-1].chars() {
        n *= 16;
        n += c.to_digit(16).unwrap() as i64;
    }
    (d,n)
}

fn run2(input: &str) -> i64 {
    use Dir::*;

    let mut vertices: Vec<(i64,i64)> = vec![(0,0)];
    let mut boundary = 0;
    for line in input.lines() {
        let line = line.rsplit_once(' ').unwrap().1.strip_prefix("(#").unwrap().strip_suffix(')').unwrap();
        let (d,n) = parse_num(line);
        boundary += n;
        let (i,j) = vertices.last().unwrap();
        match d {
            Up => vertices.push(((i-n),*j)),
            Down => vertices.push(((i+n),*j)),
            Left => vertices.push((*i,(j - n))),
            Right => vertices.push((*i,(j + n))),
        }
    }
    // Shoelace formula - A = s / 2
    let s: i64 = vertices.iter().zip(vertices.iter().skip(1)).map(|((x1,y1), (x2,y2))| x1*y2 - y1*x2).sum::<i64>().abs();
    // Pick's theorem: 2*A = 2*i + b - 2,
    // Where i is the number of internal nodes and
    // b is the number of boundary nodes
    let interior = (s - boundary) / 2 + 1;

    interior + boundary
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
    assert_eq!(res,62);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,58550);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,952408144115);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
