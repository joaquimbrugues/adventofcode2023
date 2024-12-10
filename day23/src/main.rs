use std::{env,fs,process};
use std::collections::{HashSet,HashMap,VecDeque};

#[derive(PartialEq,Eq)]
enum Dir { N, S, W, E, }

enum Path {
    Flat,
    Slope(Dir),
}

impl Path {
    fn from_char(c: char, part2: bool) -> Option<Self> {
        match c {
            '.' => Some(Self::Flat),
            '^' => {
                if part2 {
                    Some(Self::Flat)
                } else {
                    Some(Self::Slope(Dir::N))
                }
            },
            '<' => {
                if part2 {
                    Some(Self::Flat)
                } else {
                    Some(Self::Slope(Dir::W))
                }
            },
            '>' => {
                if part2 {
                    Some(Self::Flat)
                } else {
                    Some(Self::Slope(Dir::E))
                }
            },
            'v' => {
                if part2 {
                    Some(Self::Flat)
                } else {
                    Some(Self::Slope(Dir::S))
                }
            },
            '#' => None,
            _ => panic!("Unexpected character {c}"),
        }
    }

    fn neighbours(&self, coord: (isize, isize)) -> Vec<((isize, isize), Dir)> {
        match self {
            Self::Flat => vec![((coord.0 - 1, coord.1), Dir::N), ((coord.0 + 1, coord.1), Dir::S), ((coord.0, coord.1 - 1), Dir::W), ((coord.0, coord.1 + 1), Dir::E)],
            Self::Slope(dir) => match dir {
                Dir::N => vec![((coord.0 - 1, coord.1), Dir::N)],
                Dir::S => vec![((coord.0 + 1, coord.1), Dir::S)],
                Dir::W => vec![((coord.0, coord.1 - 1), Dir::W)],
                Dir::E => vec![((coord.0, coord.1 + 1), Dir::E)],
            }
        }
    }
}

#[derive(Debug)]
struct Graph {
    adj: HashMap<(isize, isize), Vec<(isize,isize)>>,
    start: (isize, isize),
    end: (isize, isize),
}

impl Graph {
    fn neighbours(&self, p: &(isize, isize)) -> &Vec<(isize, isize)> {
        self.adj.get(p).unwrap()
    }
    
    fn read_graph(input: &str, part2: bool) -> Self {
        let mut start = (0,0);
        let mut map = HashMap::new();
        for (i, line) in input.lines().enumerate() {
            let i = i as isize;
            for (j, c) in line.chars().enumerate() {
                let j = j as isize;
                if let Some(p) = Path::from_char(c, part2) {
                    map.insert((i,j), p);
                    if i == 0 {
                        start = (0, j);
                    }
                }
            }
        }
        let end = map.keys().fold((0,0), |acc, &(i, j)| {
            if acc.0 < i {
                (i, j)
            } else {
                acc
            }
        });

        let mut adj = HashMap::new();
        for (&coord, path) in map.iter() {
            let neighs = path.neighbours(coord);
            let mut ns = vec![];
            for (n, d) in neighs {
                if let Some(kind) = map.get(&n) {
                    match kind {
                        Path::Flat => ns.push(n),
                        Path::Slope(dd) => {
                            if d == *dd {
                                ns.push(n);
                            }
                        }
                    }
                }
            }
            adj.insert(coord, ns);
        }

        Self { adj, start, end, }
    }
}

fn run1(input: &str) -> usize {
    let graph = Graph::read_graph(input, false);
    let visited = HashSet::new();
    let mut stack = vec![(graph.start, visited)];
    let mut max = 0;
    while let Some((node, mut visited)) = stack.pop() {
        if node == graph.end {
            if visited.len() > max {
                max = visited.len();
            }
        } else if !visited.contains(&node) {
            visited.insert(node);
            for n in graph.neighbours(&node) {
                stack.push((*n, visited.clone()));
            }
        }
    }
    max
}

struct CondensedGraph {
    adj: HashMap<(isize, isize), Vec<((isize,isize), usize)>>,  // The values are lists of
                                                                // neighbours with distances
    start: (isize, isize),
    end: (isize, isize),
}

impl CondensedGraph {
    fn neighbours(&self, p: &(isize, isize)) -> &Vec<((isize, isize), usize)> {
        self.adj.get(p).unwrap()
    }

    fn condense(orig: Graph) -> Self {
        // Consume the original graph and return the condensed graph
        let start = orig.start;
        let end = orig.end;
        let mut adjacencies = HashMap::with_capacity(orig.adj.len());
        let mut stack_to_check = vec![end, start];
        while let Some(node) = stack_to_check.pop() {
            if !adjacencies.contains_key(&node) {
                let mut res = vec![];
                let mut queue = VecDeque::from([(node, 0)]);
                let mut visited = HashSet::from([node]);
                while let Some((n, d)) = queue.pop_front() {
                    let neighs = orig.neighbours(&n);
                    if n != node && (neighs.len() > 2 || n == end) {
                        res.push((n, d));
                        stack_to_check.push(n);
                    } else {
                        for nn in neighs {
                            if !visited.contains(nn) {
                                visited.insert(*nn);
                                queue.push_back((*nn, d+1));
                            }
                        }
                    }
                }
                adjacencies.insert(node, res);
            }
        }
        Self { adj: adjacencies, start, end, }
    }

    fn max_connectivity(&self) -> usize {
        self.adj.values().map(|v| v.len()).max().unwrap()
    }
}

fn run2(input: &str) -> usize {
    let graph = CondensedGraph::condense(Graph::read_graph(input, true));
    let visited = HashSet::new();
    let mut queue = VecDeque::from([(graph.start, visited, 0)]);
    queue.reserve(graph.max_connectivity());
    let mut max = 0;
    while let Some((node, visited, dist)) = queue.pop_front() {
        if node == graph.end {
            if dist > max {
                max = dist;
            }
        } else {
            for &(n, d) in graph.neighbours(&node) {
                if !visited.contains(&n) {
                    let mut nv = visited.clone();
                    nv.insert(n);
                    queue.push_back((n, nv, dist + d));
                }
            }
        }
    }
    max
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
    assert_eq!(res, 94);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 2334);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 154);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 6422);
}
