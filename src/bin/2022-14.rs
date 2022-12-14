use itertools::Itertools;
use std::collections::HashSet;

#[aoc::main]
fn main() {
    let mut points: HashSet<_> = input
        .lines()
        .flat_map(|l| {
            l.split(" -> ")
                .map(&coord)
                .tuple_windows()
                .flat_map(|([x, y], [xx, yy])| {
                    smart_range(x, xx).cartesian_product(smart_range(y, yy))
                })
        })
        .collect();
    let abyss = *points.iter().map(|(_x, y)| y).max().unwrap();
    let floor = abyss + 2;
    let mut fst = None;
    let mut n = 0;
    let source = (500, 0);
    while !points.contains(&source) {
        let (mut x, mut y) = source;
        loop {
            y += 1;
            if y > abyss {
                fst = fst.or(Some(n));
            }
            if !points.contains(&(x, y)) && y < floor {
                continue;
            }
            if !points.contains(&(x - 1, y)) && y < floor {
                x -= 1;
                continue;
            }
            if !points.contains(&(x + 1, y)) && y < floor {
                x += 1;
                continue;
            }
            points.insert((x, y - 1));
            n += 1;
            break;
        }
    }
    (fst.unwrap(), n)
}

fn coord(s: &str) -> [usize; 2] {
    let mut parts = s.split(',').flat_map(|n| n.parse());
    [parts.next().unwrap(), parts.next().unwrap()]
}

fn smart_range<T: Ord>(from: T, to: T) -> std::ops::RangeInclusive<T> {
    match from.cmp(&to) {
        std::cmp::Ordering::Greater => to..=from,
        _ => from..=to,
    }
}
