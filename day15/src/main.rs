use std::{env,fs,process};

fn run1(input: &str) -> usize {
    let mut res = 0;
    for code in input.split(',') {
        let code = code.trim();
        res += hash(code);
    }
    res
}

fn hash(code: &str) -> usize {
    let mut h = 0;
    for b in code.as_bytes() {
        h += *b as usize;
        h *= 17;
        h %= 256;
    }
    h
}

fn run2(input: &str) -> usize {
    let mut map: [Vec<(&str, u8)>; 256] = std::array::from_fn(|_| Vec::new());
    for code in input.split(',') {
        let code = code.trim();
        if let Some((label, snum)) = code.split_once('=') {
            let h = hash(label);
            let focal = snum.parse().unwrap();
            let mut placed = false;
            for (l, n) in map[h].iter_mut() {
                if l == &label {
                    *n = focal;
                    placed = true;
                    break;
                }
            }
            if !placed {
                map[h].push((label, focal));
            }
        } else if let Some(label) = code.strip_suffix('-') {
            let h = hash(label);
            map[h].retain(|(l,_)| l != &label);
        } else {
            panic!("Unknown operation! {code}")
        }
    }

    let mut res = 0;
    for (bn, v) in map.iter().enumerate() {
        for (sn, (_, fl)) in v.iter().enumerate() {
            res += (bn + 1) * (sn + 1) * (*fl as usize);
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
    assert_eq!(res, 1320);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,517965);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 145);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,267372);
}
