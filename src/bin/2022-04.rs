#[aoc::main]
fn main() {
    let parsed: Vec<Vec<(isize, isize)>> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|r| r.split('-').flat_map(str::parse).collect())
                .map(|e: Vec<isize>| (e[0], e[1]))
                .collect()
        })
        .collect();
    let filt = |(ds, de): &(isize, isize)| (ds.signum() + de.signum()).abs() <= 1;
    let fst = parsed
        .iter()
        .cloned()
        .flat_map(|e| e.into_iter().reduce(|e0, e1| (e0.0 - e1.0, e0.1 - e1.1)))
        .filter(filt)
        .count();
    let snd = parsed
        .into_iter()
        .flat_map(|e| e.into_iter().reduce(|e0, e1| (e0.0 - e1.1, e0.1 - e1.0)))
        .filter(filt)
        .count();
    (fst, snd)
}
