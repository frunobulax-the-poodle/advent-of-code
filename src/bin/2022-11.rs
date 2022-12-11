use itertools::*;
use regex::Regex;
use std::str::FromStr;

#[aoc::main]
fn main() {
    let mut monkes = Monke::parse_all(&input);
    let mut monkeys = monkes.clone();
    (solve(&mut monkes, 20, 3), solve(&mut monkeys, 10_000, 1))
}

fn solve(monkes: &mut [Monke], rounds: usize, div: usize) -> usize {
    for _ in 0..rounds {
        round(monkes, div);
    }
    monkes
        .iter()
        .map(|m| m.inspections)
        .sorted_unstable_by(|m1, m2| Ord::cmp(&m2, &m1))
        .take(2)
        .product()
}

fn round(monkes: &mut [Monke], div: usize) {
    let modulo: usize = monkes.iter().map(|m| m.test_div).product();
    for i in 0..monkes.len() {
        // Can't mutate items in the vec while another is mutably borrowed...
        let items: Vec<_> = monkes[i].items.drain(..).collect();
        monkes[i].inspections += items.len();
        items.iter().for_each(|old| {
            let new = (monkes[i].op.apply(*old) / div) % modulo;
            let target = monkes[i].target[(new % monkes[i].test_div == 0) as usize];
            monkes[target].items.push(new);
        });
    }
}

#[derive(Debug, Clone)]
struct Monke {
    items: Vec<usize>,
    op: Op,
    test_div: usize,
    target: [usize; 2],
    inspections: usize,
}

#[derive(Debug, Clone)]
enum Op {
    Add(Option<usize>),
    Mul(Option<usize>),
}
impl Op {
    fn apply(&self, n: usize) -> usize {
        match self {
            Op::Add(m) => n + m.unwrap_or(n),
            Op::Mul(m) => n * m.unwrap_or(n),
        }
    }
}

impl Monke {
    fn parse_all(s: &str) -> Vec<Monke> {
        let re = Regex::new(
            r"(?x)
                       .*\n
                       .*:\ (?P<items>(?:\d+(?:,\s)?)+)\n
                       .*old\ (?P<op>.*)\n
                       .*by\ (?P<test>\d+)\n
                       .*monkey\ (?P<true>\d+)\n
                       .*monkey\ (?P<false>\d+)",
        )
        .unwrap();
        re.captures_iter(s)
            .map(|cap| {
                let items_str = cap.name("items").unwrap().as_str();
                let items = items_str.split(", ").flat_map(|n| n.parse()).collect();
                let op = cap.name("op").unwrap().as_str().parse().unwrap();
                let test_div = cap.name("test").unwrap().as_str().parse().unwrap();
                let ttrue = cap.name("true").unwrap().as_str().parse().unwrap();
                let tfalse = cap.name("false").unwrap().as_str().parse().unwrap();
                Monke {
                    items,
                    op,
                    test_div,
                    target: [tfalse, ttrue],
                    inspections: 0,
                }
            })
            .collect()
    }
}

impl FromStr for Op {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arg = s[2..].parse().ok();
        match s.chars().next() {
            Some('+') => Ok(Op::Add(arg)),
            Some('*') => Ok(Op::Mul(arg)),
            _ => panic!("Invalid op!"),
        }
    }
}
