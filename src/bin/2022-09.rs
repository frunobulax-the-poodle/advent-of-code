use aoc::{v, VI};
use std::collections::HashSet;

#[aoc::main]
fn main() {
    let mut rope2 = Rope::<2>::new();
    let mut rope10 = Rope::<10>::new();
    input.lines().map(&parse_line).for_each(|d| {
        rope2.step(d);
        rope10.step(d);
    });

    (rope2.visited.len(), rope10.visited.len())
}

#[derive(Debug)]
struct Rope<const N: usize> {
    knots: [VI<2>; N],
    visited: HashSet<VI<2>>,
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Self {
            knots: [v![0; 2]; N],
            visited: HashSet::<VI<2>>::new(),
        }
    }

    fn step(&mut self, delta_head: VI<2>) {
        let target = self.knots[0] + delta_head;
        while self.knots[0] != target {
            self.knots[0] += delta_head.unit();
            for i in 0..N - 1 {
                let (head, tail) = (self.knots[i], &mut self.knots[i + 1]);
                let diff = head - *tail;
                if diff.0.iter().any(|n| n.abs() > 1) {
                    *tail += diff.unit();
                }
            }
            self.visited.insert(self.knots[N - 1]);
        }
    }
}

fn parse_line(line: &str) -> VI<2> {
    let n: isize = line[2..].parse().unwrap();
    let dir = match line.as_bytes()[0] as char {
        'R' => v![1, 0],
        'L' => v![-1, 0],
        'U' => v![0, 1],
        'D' => v![0, -1],
        _ => panic!("aaaaaaa"),
    };
    dir * n
}
