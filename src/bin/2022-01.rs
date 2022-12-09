#[aoc::main]
fn main() {
    let mut elves = input.lines().fold(vec![0], |mut acc, string| {
        match string {
            "" => acc.push(0),
            _ => *acc.last_mut().unwrap() += string.parse::<usize>().unwrap(),
        }
        acc
    });
    elves.sort_unstable();
    elves.reverse();

    (elves[0], elves[0..3].iter().sum::<usize>())
}
