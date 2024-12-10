use std::{env,fs,process};

#[derive(Debug)]
struct Trajectory2D {
    base: (f64, f64),
    vec: (f64, f64),
}

impl Trajectory2D {
    fn new(base: (f64, f64), vec: (f64, f64)) -> Self {
        Self { base, vec, }
    }

    fn intersection_forward(&self, other: &Self) -> Option<(f64, f64)> {
        let det = (self.vec.1 * other.vec.0) - (self.vec.0 * other.vec.1);
        if det == 0.0 {
            None
        } else {
            let t = ( ( other.vec.1 * ( self.base.0 - other.base.0 ) ) + ( other.vec.0 * ( other.base.1 - self.base.1 ) ) ) / det;
            let s = ( ( self.vec.1 * ( self.base.0 - other.base.0 ) ) + ( self.vec.0 * ( other.base.1 - self.base.1 ) ) ) / det;
            if t > 0.0 && s > 0.0 {
                Some( ( self.base.0 + (t * self.vec.0), self.base.1 + (t * self.vec.1) ) )
            } else {
                None
            }
        }
    }
}

fn run1(input: &str) -> u32 {
    let (first, second) = input.split_once("\n\n").unwrap();
    let (s1, s2) = first.split_once(", ").unwrap();
    let lower_bound: f64 = s1.parse().unwrap();
    let upper_bound: f64 = s2.parse().unwrap();
    let trajectories: Vec<Trajectory2D> = second.lines().map(|line| {
        let (sp, sv) = line.split_once(" @ ").unwrap();
        let p: Vec<f64> = sp.split(", ").map(|tok| tok.trim().parse::<i64>().unwrap() as f64).collect();
        let v: Vec<f64> = sv.split(", ").map(|tok| tok.trim().parse::<i64>().unwrap() as f64).collect();
        Trajectory2D::new((p[0], p[1]), (v[0], v[1]))
    }).collect();

    let mut res = 0;
    for i in 0..trajectories.len() {
        for j in (i+1)..trajectories.len() {
            if let Some(int) = trajectories[i].intersection_forward(&trajectories[j]) {
                if lower_bound <= int.0 && int.0 <= upper_bound && lower_bound <= int.1 && int.1 <= upper_bound {
                    res += 1;
                }
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
    assert_eq!(res, 2);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 24192);
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
