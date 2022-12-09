use std::str::FromStr;

#[aoc::main]
fn main() {
    let mut it = input.split("\n\n");
    let mut crates: Crates = it.next().unwrap().parse().unwrap();
    let instrs: Vec<Instr> = it.next().unwrap().lines().flat_map(str::parse).collect();
    let mut crates2 = crates.clone();

    (crates.solve(&instrs, true), crates2.solve(&instrs, false))
}

#[derive(Debug, Clone)]
struct Crates(Vec<Vec<char>>);

impl Crates {
    fn solve(&mut self, instrs: &[Instr], rev: bool) -> String {
        instrs.iter().for_each(|it| self.apply_one(it, rev));
        self.0.iter().map(|v| v.last().unwrap()).collect::<String>()
    }
    fn apply_one(&mut self, instr: &Instr, rev: bool) {
        let from = &mut self.0[instr.from];
        let drained = from.drain(from.len() - instr.n..);
        let mut order = if rev {
            drained.rev().collect()
        } else {
            drained.collect()
        };
        self.0[instr.to].append(&mut order);
    }
}

struct Instr {
    n: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instr {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<usize> = s
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .flat_map(str::parse)
            .collect();
        Ok(Instr {
            n: parts[0],
            from: parts[1] - 1,
            to: parts[2] - 1,
        })
    }
}

impl FromStr for Crates {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut crates = Crates(Vec::new());
        s.lines().rev().skip(1).for_each(|l| {
            l.chars().skip(1).step_by(4).enumerate().for_each(|(i, c)| {
                if crates.0.get(i).is_none() {
                    crates.0.push(Vec::new());
                }
                if !c.is_whitespace() {
                    crates.0[i].push(c);
                }
            })
        });
        Ok(crates)
    }
}
