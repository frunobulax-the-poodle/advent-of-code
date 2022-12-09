#![feature(iter_array_chunks)]
use std::collections::HashSet;

#[aoc::main]
fn main() {
    let fst: usize = input
        .lines()
        .map(|l| {
            *l[0..l.len() / 2]
                .chars()
                .collect::<HashSet<char>>()
                .intersection(&l[l.len() / 2..l.len()].chars().collect())
                .next()
                .unwrap()
        })
        .map(prio)
        .sum();

    let snd: usize = input
        .lines()
        .map(|l| l.chars().collect::<HashSet<char>>())
        .array_chunks::<3>()
        .map(|bags| {
            *bags
                .into_iter()
                .reduce(|acc, b| acc.intersection(&b).cloned().collect())
                .unwrap()
                .iter()
                .next()
                .unwrap()
        })
        .map(prio)
        .sum();
    (fst, snd)
}

fn prio(c: char) -> usize {
    if c.is_lowercase() {
        1 + c as usize - 'a' as usize
    } else {
        27 + c as usize - 'A' as usize
    }
}
