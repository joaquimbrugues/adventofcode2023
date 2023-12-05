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

// Idea: x is (init, range) and map is (dest_init, source_init, mrange).
// If [init, init + range[ intersects [source_init, source_init + mrange[, map the intersection
// with a translation of dest_init - source init, and keep the rest
fn intersect(x: (u64, u64), map: (u64, u64, u64)) -> Vec<(u64, u64)> {
    let mut res = vec![];
    if x.0 + x.1 <= map.1 || map.1 + map.2 <= x.0 {
        // No intersection
        //res.push(x);
    } else if x.0 <= map.1 {
        // Intersection, with init to the left or equal to source_init
        let r1 = map.1 - x.0;
        if r1 > 0 {
            res.push((x.0, r1));
        }
        if x.0 + x.1 <= map.1 + map.2 {
            // "Clean" intersection, with init+range to the left or equal to source_init+mrange,
            // only map the intersection
            res.push((map.0, x.0 + x.1 - map.1));
        } else {
            // [source_init, source_init + mrange[ is contained within [init, init+range[
            // Intersection
            res.push((map.0, map.2));
            // Remainder
            res.push((map.1 + map.2, x.0 + x.1 - map.1 - map.2));
        }
    } else {
        // Intersection, with init strictly to the right of source_init
        if x.0 + x.1 <= map.1 + map.2 {
            // [init, init + range[ is contained within [source_init, source_init + mrange[
            // Map the intersection
            res.push((map.0 + x.0 - map.1, x.1));
        } else {
            // "Clean" intersection, with init + range strictly to the right  of source_init +
            // mrange
            // Map the intersection
            res.push((map.0 + x.0 - map.1, map.1 + map.2 - x.0));
            // Keep the remainder
            res.push((map.1 + map.2, x.0 + x.1 - map.1 - map.2));
        }
    }
    res
}

fn run2(input: &str) -> u64 {
    let (head, tail) = input.split_once("\n\n").unwrap();
    let (_, head) = head.split_once(": ").unwrap();
    let mut following: HashSet<(u64, u64)> = head.split_whitespace().step_by(2).zip(head.split_whitespace().skip(1).step_by(2)).map(|(s1, s2)| (s1.parse().unwrap(), s2.parse().unwrap())).collect();
    for block in tail.split("\n\n") {
        // TODO: Problem when joining intersections together...
        let mut temp = HashSet::new();
        let mut intersects = HashSet::new();
        for line in block.lines().skip(1) {
            let map: Vec<u64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            assert_eq!(map.len(), 3);
            let map = (map[0], map[1], map[2]);
            for x in following.iter() {
                let i = intersect(*x, map);
                if i.len() > 0 {
                    intersects.insert(x);
                }
                for v in i {
                    temp.insert(v);
                }
            }
        }
        for v in following.iter().filter(|f| !intersects.contains(f)) {
            temp.insert(*v);
        }
        following = temp;
    }
    following.into_iter().map(|(i,_)| i).min().unwrap()
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
    assert_eq!(res, 35);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 379811651);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 46);
}

#[test]
fn intersection1() {
    let x = (10, 5);
    let map = (0, 15, 20);
    assert_eq!(intersect(x,map).len(), 0);
    let x = (31, 2);
    let map = (0, 40, 10000);
    assert_eq!(intersect(x,map).len(), 0);
}

#[test]
fn intersection2() {
    let x = (5, 18);
    let map = (0, 3, 2);
    assert_eq!(intersect(x,map).len(), 0);
    let x = (9, 18);
    let map = (0, 3, 2);
    assert_eq!(intersect(x,map).len(), 0);
}

#[test]
fn intersection3() {
    let x = (12, 20);
    let map = (1, 12, 21);
    assert_eq!(intersect(x,map), vec![(1, 20)]);
    let map = (0, 13, 21);
    assert_eq!(intersect(x,map), vec![(12,1), (0, 19)]);
}

#[test]
fn intersection4() {
    let x = (15, 15);
    let map = (100, 20, 5);
    assert_eq!(intersect(x,map), vec![(15,5), (100, 5), (25, 5)]);
}

#[test]
fn intersection5() {
    let x = (7, 7);
    let map = (100, 4, 7);
    assert_eq!(intersect(x,map), vec![(103,4), (11,3)]);
    let map = (100, 6, 8);
    assert_eq!(intersect(x,map), vec![(101,7)]);
}

#[test]
fn intersection6() {
    let x = (40, 10);
    let map = (0, 30, 30);
    assert_eq!(intersect(x,map), vec![(10, 10)]);
    let x = (30, 10);
    assert_eq!(intersect(x,map), vec![(0, 10)]);
    let x = (50, 10);
    assert_eq!(intersect(x,map), vec![(20, 10)]);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
