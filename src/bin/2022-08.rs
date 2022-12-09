use itertools::Itertools;
use ndarray::{s, Array2, ArrayView, Ix1};

#[aoc::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mat = parse(&input);
    (seen(&mat), best_view(&mat))
}

fn seen(mat: &Array2<usize>) -> usize {
    let (nrows, ncols) = mat.dim();
    let n_edge = nrows * 2 + ncols * 2 - 4;
    n_edge
        + mat
            .slice(s![1..-1, 1..-1])
            .indexed_iter()
            .filter(|((y, x), height)| {
                line_of_sight(mat, y + 1, x + 1)
                    .iter()
                    .any(|dir| dir.iter().all(|t| t < height))
            })
            .count()
}

fn best_view(mat: &Array2<usize>) -> usize {
    mat.indexed_iter()
        .map(|((y, x), height)| {
            line_of_sight(mat, y, x)
                .iter()
                .map(|dir| {
                    let mut it = dir.iter();
                    let seen = it.take_while_ref(|t| t < &height).count();
                    seen + it.next().is_some() as usize
                })
                .product()
        })
        .max()
        .unwrap_or(0)
}

fn parse<'a>(s: &'a str) -> Array2<usize> {
    let f = |line: &'a str| line.chars().map(|c| c.to_digit(10).unwrap_or(0) as usize);
    let mut lines = s.lines();
    let fst: Vec<_> = f(lines.next().unwrap()).collect();
    let ncols = fst.len();
    let vec: Vec<_> = fst.into_iter().chain(lines.flat_map(&f)).collect();
    let nrows = vec.len() / ncols;
    Array2::from_shape_vec((nrows, ncols), vec).unwrap()
}

fn line_of_sight(mat: &Array2<usize>, y: usize, x: usize) -> [ArrayView<usize, Ix1>; 4] {
    [
        mat.slice(s![y, x + 1..]), // right
        mat.slice(s![y, ..x; -1]), // left
        mat.slice(s![y + 1.., x]), // down
        mat.slice(s![..y; -1, x]), // up
    ]
}
