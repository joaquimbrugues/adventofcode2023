use std::{env,fs,process};
use std::collections::{HashMap,VecDeque};
use std::cell::RefCell;

// Boolean - PulseType
// false   - low
// true    - high
#[derive(Debug,PartialEq,Eq,Clone,)]
enum ModuleType<'a> {
    Broadcast,
    FlipFlop(RefCell<bool>),
    Conjunction(RefCell<HashMap<&'a str, bool>>),
}

impl<'a> ModuleType<'a> {
    fn broadcast() -> Self {
        Self::Broadcast
    }

    fn flipflop() -> Self {
        Self::FlipFlop(RefCell::new(false))
    }

    fn conjunction() -> Self {
        Self::Conjunction(RefCell::new(HashMap::new()))
    }

    fn process_signal(&'a self, signal: bool, from: &'a str) -> Option<bool> {
        match self {
            Self::Broadcast => Some(signal),
            Self::FlipFlop(cell) => {
                if !signal {
                    let state = &mut *cell.borrow_mut();
                    *state = !*state;
                    Some(*state)
                } else {
                    None
                }
            },
            Self::Conjunction(cell) => {
                let mut table = cell.borrow_mut();
                if table.insert(from, signal).is_none() {
                    panic!("Signal received from module {from}, which was unregistered for this conjuction module!");
                }
                if table.values().fold(true, |acc, &s| acc && s) {
                    Some(false)
                } else {
                    Some(true)
                }
            },
        }
    }
}

#[derive(Debug)]
struct Module<'a> {
    kind: ModuleType<'a>,
    neighs: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn new(kind: ModuleType<'a>) -> Self {
        Self { kind, neighs: Vec::new(), }
    }

    fn parse_input(input: &'a str) -> HashMap<&'a str, Self> {
        // First just collect the labels
        let mut map: HashMap<&str, Self> = input.lines().map(|line| {
            let (label, _) = line.split_once("->").unwrap();
            let label = label.trim();
            if let Some(label) = label.strip_prefix('%') {
                (label, Self::new(ModuleType::flipflop()))
            } else if let Some(label) = label.strip_prefix('&') {
                (label, Self::new(ModuleType::conjunction()))
            } else {
                (label, Self::new(ModuleType::broadcast()))
            }
        }).collect();
        assert_eq!(map.values().fold(0, |acc, m| {
            if m.kind == ModuleType::Broadcast {
                acc + 1
            } else {
                acc
            }
        }), 1);

        for line in input.lines() {
            let (label, ns) = line.split_once("->").unwrap();
            let label = if let Some(l) = label.trim().strip_prefix('%') {
                l
            } else if let Some(l) = label.trim().strip_prefix('&') {
                l
            } else {
                label.trim()
            };

            for n in ns.split(',').map(|s| s.trim()) {
                map.get_mut(label).unwrap().neighs.push(n);
                if let Some(module) = map.get(n) {
                    if let ModuleType::Conjunction(cell) = &module.kind {
                        // Initialize conjunction module
                        cell.borrow_mut().insert(label, false);
                    }
                }
            }
        }

        map
    }
}

fn run1(input: &str) -> u32 {
    let graph = Module::parse_input(input);

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back((false, "button", "broadcaster"));
        while let Some((signal, from, to)) = queue.pop_front() {
            //println!("{from} - {signal} > {to}");
            if signal {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }

            if let Some(module) = graph.get(to) {
                let signal = module.kind.process_signal(signal, from);
                if let Some(signal) = signal {
                    for n in module.neighs.iter() {
                        queue.push_back((signal, to, n));
                    }
                }
            }
        }
    }

    low_pulses * high_pulses
}

fn run2(input: &str) -> u64 {
    let graph = Module::parse_input(input);

    let precedents: Vec<_> = graph.iter().filter(|(_, module)| module.neighs.contains(&"rx")).map(|(label,_)| label).collect();
    assert_eq!(precedents.len(), 1);
    let precedent = precedents[0];
    let mut periods: HashMap<&str, u64> = graph.iter().filter(|(_, module)| module.neighs.contains(precedent)).map(|(&label, _)| (label, 0)).collect();

    let mut pulses = 0;
    while periods.values().any(|&n| n == 0) {
        pulses += 1;
        let mut queue = VecDeque::new();
        queue.push_back((false, "button", "broadcaster"));
        while let Some((signal, from, to)) = queue.pop_front() {
            //println!("{from} - {signal} > {to}");
            if signal && periods.keys().any(|l| l == &from) {
                // {precedent} was sent a positive signal.
                // Register it in periods if it was not previously registered
                //let mut count = periods.get_mut(from).unwrap();
                let count = periods.get_mut(from).unwrap();
                if *count == 0 {
                    *count = pulses;
                }
            }

            if let Some(module) = graph.get(to) {
                let signal = module.kind.process_signal(signal, from);
                if let Some(signal) = signal {
                    for n in module.neighs.iter() {
                        queue.push_back((signal, to, n));
                    }
                }
            }
        }
    }

    periods.values().product()
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

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 819397964);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 252667369442479);
}
