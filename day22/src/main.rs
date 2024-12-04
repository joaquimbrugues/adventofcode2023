use std::{env,fs,process};
use std::collections::{HashSet,HashMap,VecDeque};

fn get_support_network(input: &str) -> HashMap<usize, HashSet<usize>> {
    // Step 1: Read the input
    let snapshot: Vec<((u32,u32,u32), (u32,u32,u32))> = input.lines().map(|line| {
        let (first, second) = line.split_once('~').unwrap();
        let first: Vec<u32> = first.split(',').map(|tok| tok.parse().unwrap()).collect();
        assert_eq!(first.len(), 3);
        let second: Vec<u32> = second.split(',').map(|tok| tok.parse().unwrap()).collect();
        assert_eq!(second.len(), 3);
        ((first[0], first[1], first[2]), (second[0], second[1], second[2]))
    }).fold(vec![], |mut acc, (a,b)| {
        let mut i = 0;
        while i < acc.len() && a.2 > acc[i].0.2 { i += 1; }
        acc.insert(i, (a,b));
        acc
    });

    // Step 2: Figure out how the tower settles
    let mut tower: HashMap<(u32,u32,u32), usize> = HashMap::new();
    let mut supports: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (i, (a,b)) in snapshot.into_iter().enumerate() {
        let height = b.2 - a.2;
        let mut settlez = a.2;
        let mut falling = true;
        let mut sup = HashSet::new();
        while settlez > 1 && falling {
            for x in a.0..=b.0 {
                for y in a.1..=b.1 {
                    if let Some(id) = tower.get(&(x,y,settlez - 1)) {
                        falling = false;
                        sup.insert(id.clone());
                    }
                }
            }
            if falling { settlez -= 1; }
        }
        for x in a.0..=b.0 {
            for y in a.1..=b.1 {
                for z in settlez..=(settlez + height) {
                    tower.insert((x,y,z), i);
                }
            }
        }
        supports.insert(i, sup);
    }

    supports
}

fn run1(input: &str) -> usize {
    let supports = get_support_network(input);

    // Step 3: Remove every brick except the ones that are the only support of some other brick
    let only_support: HashSet<usize> = supports.values().filter(|set| set.len() == 1).map(|set| *set.iter().next().unwrap()).collect();
    supports.len() - only_support.len()
}

fn above(id: usize, supports: &HashMap<usize, HashSet<usize>>) -> Vec<usize> {
    supports.iter().filter(|(_, set)| set.contains(&id)).map(|(&k, _)| k).collect()
}

fn run2(input: &str) -> usize {
    let supports = get_support_network(input);
    
    supports.keys().map(|&id| {
        let mut queue = VecDeque::from(vec![id]);
        let mut fallen: HashSet<usize> = HashSet::from([id]);

        while let Some(brick) = queue.pop_front() {
            for a in above(brick, &supports) {
                if !fallen.contains(&a) {
                    if supports.get(&a).unwrap().iter().all(|b| fallen.contains(b)) {
                        // All suports fell, thus this brick falls as well!
                        fallen.insert(a);
                        queue.push_back(a);
                    }
                }
            } 
        }

        fallen.len() - 1
    }).sum()
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
    assert_eq!(res, 5);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 398);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 7);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 70727);
}
