#![feature(array_windows)]
use itertools::Itertools;

#[aoc::main]
fn main() {
    let chars: Vec<char> = input.chars().collect();
    (solve::<4>(&chars), solve::<14>(&chars))
}

fn solve<const N: usize>(chars: &[char]) -> usize {
    chars
        .array_windows::<N>()
        .enumerate()
        .find(|(_i, slice)| slice.iter().unique().count() == N)
        .unwrap()
        .0
        + N
}
