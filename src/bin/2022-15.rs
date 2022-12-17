use aoc::{v, VI};
use regex::Regex;
use std::collections::HashSet;

const ROW_1: isize = 2_000_000;
const MIN_XY: isize = 0;
const MAX_XY: isize = 4_000_000;

#[aoc::main]
fn main() {
    let sensors: Vec<Sensor> = input.lines().flat_map(|l| l.parse()).collect();
    (one(&sensors), two(&sensors))
}

fn one(sensors: &[Sensor]) -> usize {
    // Only keep sensors that even have the y coordinate in range
    let filtered: Vec<_> = sensors
        .iter()
        .filter(|s| s.pos.manhattan(&s.beacon) >= (s.pos.0[1] - ROW_1).abs())
        .collect();
    let occupied: HashSet<_> = filtered
        .iter()
        .flat_map(|s| [s.pos, s.beacon])
        .filter(|v| v.0[1] == ROW_1)
        .collect();
    let min_x = filtered
        .iter()
        .map(|s| s.pos.0[0].min(s.beacon.0[0]))
        .min()
        .unwrap();
    let max_x = filtered
        .iter()
        .map(|s| s.pos.0[0].max(s.beacon.0[0]))
        .max()
        .unwrap();
    let max_dist = filtered.iter().map(|s| s.dist).max().unwrap();

    (min_x - max_dist..=max_x + max_dist)
        .filter(|x| {
            let pos = v![*x, ROW_1];
            !occupied.contains(&pos) && filtered.iter().any(|s| s.is_in_range(&pos))
        })
        .count()
}

fn two(sensors: &[Sensor]) -> isize {
    let mut stack = vec![[v![MIN_XY, MIN_XY], v![MAX_XY, MAX_XY]]];

    while let Some([from, to]) = stack.pop() {
        if from == to {
            if sensors.iter().all(|s| !s.is_in_range(&from)) {
                return from.0[0] * MAX_XY + from.0[1];
            } else {
                continue;
            }
        }
        let center = (from + to) / 2;
        let quadrants = [
            [from, center],
            [v![center.0[0] + 1, from.0[1]], v![to.0[0], center.0[1]]],
            [v![from.0[0], center.0[1] + 1], v![center.0[0], to.0[1]]],
            [v![center.0[0] + 1, center.0[1] + 1], to],
        ];

        stack.extend(
            quadrants
                .into_iter()
                .filter(|q| q[0].0[0] <= q[1].0[0] && q[0].0[1] <= q[1].0[1])
                .filter(|q| !sensors.iter().any(|s| s.contains_quadrant(q))),
        )
    }
    panic!("aaaa");
}

#[derive(Debug)]
struct Sensor {
    pos: VI<2>,
    beacon: VI<2>,
    dist: isize,
}

impl Sensor {
    fn is_in_range(&self, v: &VI<2>) -> bool {
        self.dist >= self.pos.manhattan(v)
    }

    fn contains_quadrant(&self, [from, to]: &[VI<2>; 2]) -> bool {
        let corners = [from, to, &v![from.0[0], to.0[1]], &v![to.0[0], from.0[1]]];
        let max_dist = corners.iter().map(|c| self.pos.manhattan(c)).max().unwrap();
        max_dist < self.dist
    }
}

impl std::str::FromStr for Sensor {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Sensor, Self::Err> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
        }
        let mut iter = RE.find_iter(s).flat_map(|mat| mat.as_str().parse());
        let pos = v![iter.next().unwrap(), iter.next().unwrap()];
        let beacon = v![iter.next().unwrap(), iter.next().unwrap()];
        Ok(Sensor {
            pos,
            beacon,
            dist: pos.manhattan(&beacon),
        })
    }
}
