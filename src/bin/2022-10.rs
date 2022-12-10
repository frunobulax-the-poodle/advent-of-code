use std::str::FromStr;

#[aoc::main]
fn main() {
    let instrs: Vec<Instr> = input.lines().flat_map(|l| l.parse()).collect();
    let mut cpu = Cpu::new();
    let (fst, bools) = cpu.solve(&instrs);
    (fst, format_bools(&bools))
}

struct Cpu {
    x: isize,
    cycle: usize,
}

impl Cpu {
    fn solve(&mut self, instrs: &[Instr]) -> (isize, Vec<bool>) {
        let mut bools = vec![];
        let fst = instrs.iter().fold(0, |acc, instr| {
            (0..instr.cycle).for_each(|_| {
                bools.push((self.x - 1..=self.x + 1).contains(&(bools.len() as isize % 40)))
            });

            let next_cycle = self.cycle + instr.cycle;
            let n = if self.cycle % 40 < 20 && 20 <= next_cycle % 40 {
                next_cycle - next_cycle % 10
            } else {
                0
            } as isize
                * self.x;
            self.instr(instr);
            acc + n
        });
        (fst, bools)
    }

    fn instr(&mut self, instr: &Instr) {
        self.x += instr.add;
        self.cycle += instr.cycle;
    }

    fn new() -> Self {
        Cpu { x: 1, cycle: 0 }
    }
}

fn format_bools(bools: &[bool]) -> String {
    bools
        .iter()
        .map(|b| if *b { '█' } else { '░' })
        .enumerate()
        .flat_map(|(i, c)| if i % 40 == 0 { vec!['\n', c] } else { vec![c] })
        .collect::<String>()
}

struct Instr {
    add: isize,
    cycle: usize,
}

impl FromStr for Instr {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..4] {
            "addx" => Ok(Instr {
                add: s[5..].parse().unwrap(),
                cycle: 2,
            }),
            "noop" => Ok(Instr { add: 0, cycle: 1 }),
            _ => panic!("aaaa"),
        }
    }
}
