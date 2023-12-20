use std::{env,fs,process};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq,)]
enum Dest<'a> {
    A,  // Accept
    R,  // Reject
    Wf(&'a str), // Workflow
}

impl<'a> Dest<'a> {
    fn parse(string: &'a str) -> Self {
        match string {
            "A" => Self::A,
            "R" => Self::R,
            _ => Self::Wf(string),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
enum Cond {
    Lt(usize,u32),
    Gt(usize,u32),
    Void,
}

impl Cond {
    fn parse<'a>(string: &'a str, xmas: &HashMap<char, usize>) -> (Self, Dest<'a>) {
        if let Some((cond, label)) = string.split_once(':') {
            let dest = Dest::parse(label);
            if let Some((first, second)) = cond.split_once('>') {
                (Self::Gt( *xmas.get(&first.chars().nth(0).unwrap()).unwrap(), second.parse().unwrap() ), dest)
            } else if let Some((first, second)) = cond.split_once('<') {
                (Self::Lt( *xmas.get(&first.chars().nth(0).unwrap()).unwrap(), second.parse().unwrap() ), dest)
            } else {
                unreachable!()
            }
        } else {
            (Self::Void, Dest::parse(string))
        }
    }

    fn compare(&self, marks: &[u32;4]) -> bool {
        match *self {
            Self::Void => true,
            Self::Lt(i,n) => marks[i] < n,
            Self::Gt(i,n) => marks[i] > n,
        }
    }
}

fn run1(input: &str) -> u32 {
    let xmas = HashMap::from([('x',0), ('m',1), ('a', 2), ('s', 3)]);
    let (rules, items) = input.split_once("\n\n").unwrap();
    let rules: HashMap<_, _> = rules.lines().map(|line| {
        let (label, line) = line.split_once('{').unwrap();
        let line = line.strip_suffix('}').unwrap();
        let v: Vec<_> = line.split(',').map(|string| Cond::parse(string, &xmas)).collect();
        (label, v)
    }).collect();
    assert!(rules.contains_key("in"));

    items.lines().map(|line| {
        let line = line.strip_prefix('{').unwrap();
        let line = line.strip_suffix('}').unwrap();
        let mut marks = [0;4];
        for s in line.split(',') {
            let (l, n) = s.split_once('=').unwrap();
            assert_eq!(l.len(), 1);
            marks[*xmas.get(&l.chars().nth(0).unwrap()).unwrap()] = n.parse().unwrap();
        }
        marks
    }).filter(|marks| {
        let mut label = "in";
        loop {
            let rulev = rules.get(label).unwrap();
            for (cond, target) in rulev {
                if cond.compare(marks) {
                    match target {
                        Dest::A => return true,
                        Dest::R => return false,
                        Dest::Wf(s) => label = s,
                    }
                    break;
                }
            }
        }
    }).map(|marks| {
        marks[0] + marks[1] + marks[2] + marks[3]
    }).sum()
}

fn run2(input: &str) -> usize {
    let xmas = HashMap::from([('x',0), ('m',1), ('a', 2), ('s', 3)]);
    let (rules, _) = input.split_once("\n\n").unwrap();
    let rules: HashMap<_, _> = rules.lines().map(|line| {
        let (label, line) = line.split_once('{').unwrap();
        let line = line.strip_suffix('}').unwrap();
        let v: Vec<_> = line.split(',').map(|string| Cond::parse(string, &xmas)).collect();
        (label, v)
    }).collect();
    assert!(rules.contains_key("in"));

    //TODO: Must find a way to trim down time!
    let mut res = 0;
    for x in 1..=4000 {
        for m in 1..=4000 {
            for a in 1..=4000 {
                for s in 1..=4000 {
                    let mut label = "in";
                    let mut reject = false;
                    while !reject {
                        for (cond, target) in rules.get(label).unwrap() {
                            if cond.compare(&[x,m,a,s]) {
                                match target {
                                    Dest::A => {
                                        res += 1;
                                        reject = true;
                                    },
                                    Dest::R => reject = true,
                                    Dest::Wf(r) => label = r,
                                }
                                break;
                            }
                        }
                    }
                    println!("({x}, {m}, {a}, {s})");
                }
            }
        }
    }
    res

    //(1..=4000).zip(1..=4000)
        //.zip(1..=4000)
        //.zip(1..=4000)
        //.map(|(((x,m),a),s)| [x,m,a,s])
        //.filter(|marks| {
            //let mut label = "in";
            //loop {
                //let rulev = rules.get(label).unwrap();
                //for (cond, target) in rulev {
                    //if cond.compare(marks) {
                        //match target {
                            //Dest::A => return true,
                            //Dest::R => return false,
                            //Dest::Wf(s) => label = s,
                        //}
                        //break;
                    //}
                //}
            //}
        //})
        //.count()
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
    assert_eq!(res, 19114);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 348378);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 167409079868000);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
