use std::{env,fs,process};
use std::collections::{HashMap,VecDeque};

// Boolean - PulseType
// false   - low
// true    - high

#[derive(Debug,PartialEq, Eq, Clone,)]
enum ModuleType<'a> {
    FlipFlop(bool),
    Conj(HashMap<&'a str, bool>),
    Broadcast,
}

impl<'a> ModuleType<'a> {
    fn process_signal(&'a mut self, signal: bool, origin: &'a str) -> Option<bool> {
        match self {
            Self::FlipFlop(state) => {
                if signal {
                    None
                } else {
                    *state = !*state;
                    Some(*state)
                }
            },
            Self::Conj(memory) => {
                memory.insert(origin, signal);
                if memory.values().fold(true, |acc, b| acc & b) {
                    Some(false)
                } else {
                    Some(true)
                }
            },
            Self::Broadcast => Some(signal),
        }
    }

    fn is_conj(&self) -> bool {
        match self {
            Self::Conj(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug,)]
struct Module<'a> {
    kind: ModuleType<'a>,
    dests: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn parse(line: &'a str) -> (&'a str, Self) {
        let (head, tail) = line.split_once("->").unwrap();
        let head = head.trim();
        let dests = tail.split(',').map(|s| s.trim()).collect();
        let kind = if let Some(head) = head.strip_prefix('%') {
            // Flip-Flop
            ModuleType::FlipFlop(false)
        } else if let Some(head) = head.strip_prefix('&') {
            // Conjunction
            ModuleType::Conj(HashMap::new())
        } else {
            // Broadcast
            ModuleType::Broadcast
        };
        (head, Self { kind, dests, })
    }
}

fn parse_input(input: &str) -> HashMap<&str, Module> {
    let mut graph: HashMap<_,_> = input.lines().map(|line| Module::parse(line)).collect();
    // Assert that the graph has exactly one broadcaster module
    assert_eq!(graph.values().filter(|module| module.kind == ModuleType::Broadcast).count(), 1);
    // Initialize conjugator modules

    graph
}

fn run1(input: &str) -> u32 {
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
fn example11() {
    let input = fs::read_to_string("test1.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 32000000);
}

#[test]
fn example12() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 11687500);
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
