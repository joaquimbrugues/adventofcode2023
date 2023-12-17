use std::{env,fs,process};
use std::collections::{HashSet,HashMap,BinaryHeap};
use std::cmp;

#[derive(Debug,PartialEq,Eq,Clone,Copy,Hash)]
enum Orientation { Vert, Hori, }

impl Orientation {
    fn opp(&self) -> Self {
        use Orientation::*;
        match self {
            Vert => Hori,
            Hori => Vert,
        }
    }

    fn advance(&self, pt: (isize, isize)) -> [(isize, isize); 6] {
        use Orientation::*;
        match self {
            Vert => [
                (pt.0 - 3, pt.1),
                (pt.0 - 2, pt.1),
                (pt.0 - 1, pt.1),
                (pt.0 + 1, pt.1),
                (pt.0 + 2, pt.1),
                (pt.0 + 3, pt.1),
            ],
            Hori => [
                (pt.0, pt.1 - 3),
                (pt.0, pt.1 - 2),
                (pt.0, pt.1 - 1),
                (pt.0, pt.1 + 1),
                (pt.0, pt.1 + 2),
                (pt.0, pt.1 + 3),
            ],
        }
    }
}

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
struct Node {
    x: isize,
    y: isize,
    loss: i32,
    ori: Orientation,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.loss.cmp(&other.loss)
    }
}

impl From<(isize,isize,i32,Orientation)> for Node {
    fn from(val: (isize, isize, i32, Orientation)) -> Self {
        Self { x: val.0, y: val.1, loss: val.2, ori: val.3, }
    }
}

fn run1(input: &str) -> i32 {
    let mut map = HashMap::new();
    let mut height: isize = 0;
    let mut width: isize = 0;
    for (i,line) in input.lines().enumerate() {
        let i = i as isize;
        for (j, c) in line.chars().enumerate() {
            let j = j as isize;
            map.insert((i,j), - (c.to_digit(10).unwrap() as i32));
            if j > width {
                width = j;
            }
        }
        if i > height {
            height = i;
        }
    }
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push(Node::from((0,0,0,Orientation::Hori)));
    heap.push(Node::from((0,0,0,Orientation::Vert)));
    visited.insert((0,0,Orientation::Hori));
    visited.insert((0,0,Orientation::Vert));

    while let Some(node) = heap.pop() {
        println!("{node:?}");
        if node.x == height - 1 && node.y == width - 1 {
            return - node.loss;
        }

        let mut acc_loss = 0;
        //TODO: Add elements to the heap, accumulate loss
    }
    0
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
    assert_eq!(res, 102);
}

//#[test]
//fn input1() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run1(&input);
    //assert_eq!(res,42);
//}

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
