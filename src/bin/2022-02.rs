#[aoc::main]
fn main() {
    input
        .lines()
        .map(|line| line.split_whitespace().map(parse).collect())
        .fold((0, 0), |(a, b), shapes: Vec<isize>| {
            (
                a + score(shapes[0], shapes[1]),
                b + score(
                    shapes[0],
                    (shapes[0] + (shapes[1] - 1).signum()).rem_euclid(3),
                ),
            )
        })
}

fn score(them: isize, you: isize) -> isize {
    1 + you + (1 + you - them).rem_euclid(3) * 3
}

fn parse(s: &str) -> isize {
    match s.chars().next().unwrap() {
        'A' | 'X' => 0,
        'B' | 'Y' => 1,
        _ => 2,
    }
}
