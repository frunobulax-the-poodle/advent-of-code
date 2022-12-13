use std::cmp::Ordering;
use std::str::FromStr;
use Packet::*;

#[aoc::main]
fn main() {
    let pairs: Vec<Vec<_>> = input
        .split("\n\n")
        .map(|s| s.lines().flat_map(|l| l.parse::<Packet>()).collect())
        .collect();
    let fst: usize = pairs
        .iter()
        .map(|v| v[0].cmp(&v[1]))
        .enumerate()
        .filter_map(|(i, ord)| Some(i + 1).filter(|_| ord.is_lt()))
        .sum();
    let mut packets: Vec<Packet> = pairs.into_iter().flatten().collect();
    packets.sort();
    let a = List(vec![List(vec![Int(2)])]);
    let b = List(vec![List(vec![Int(6)])]);
    let pos1 = packets
        .iter()
        .position(|pack| pack.cmp(&a).is_gt())
        .unwrap();
    let pos2 = packets[pos1..]
        .iter()
        .position(|pack| pack.cmp(&b).is_gt())
        .unwrap();
    (fst, (pos1 + 1) * (2 + pos1 + pos2))
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Int(usize),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Int(a), Int(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (a @ Int(_), List(b)) => std::slice::from_ref(a).cmp(b.as_slice()),
            (List(a), b @ Int(_)) => a.as_slice().cmp(std::slice::from_ref(b)),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Packet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        let mut curr = Vec::new();
        let bytes = s.as_bytes();
        let mut i = 0;
        while let Some(b) = bytes.get(i) {
            match b {
                b'[' => {
                    stack.push(curr);
                    curr = Vec::new();
                    i += 1;
                }
                b']' => {
                    let packet = List(curr);
                    curr = stack.pop().expect("Empty stack!");
                    curr.push(packet);
                    i += 1;
                }
                b'0'..=b'9' => {
                    let pos = i
                        + 1
                        + bytes[i + 1..]
                            .iter()
                            .position(|b| !b.is_ascii_digit())
                            .unwrap_or(bytes.len() - 1);
                    curr.push(Int(s[i..pos].parse().unwrap()));
                    i = pos;
                }
                b',' => i += 1,
                _ => panic!("Unexpected char `{}`", b),
            }
        }
        assert!(stack.is_empty(), "unclosed delimiter!");
        Ok(curr.pop().unwrap())
    }
}
