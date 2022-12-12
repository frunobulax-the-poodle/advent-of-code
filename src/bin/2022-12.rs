#![feature(return_position_impl_trait_in_trait)]
use ndarray::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;

#[aoc::main]
fn main() {
    let map: Map = input.parse().unwrap();
    let fst = map.a_star(map.start, map.goal);
    let snd = map
        .map
        .indexed_iter()
        .filter_map(|(ix, height)| Some(ix).filter(|_| height == &0))
        .map(|(y, x)| map.a_star(Ix2(y, x), map.goal).len())
        .filter(|it| it > &0)
        .min()
        .unwrap()
        - 1;
    (fst.len() - 1, snd)
}

#[derive(Debug)]
struct Map {
    map: Array2<u8>,
    start: Ix2,
    goal: Ix2,
}

trait AStar {
    type Node: Eq + std::hash::Hash + Debug + Copy;

    fn weight(&self, from: &Self::Node, to: &Self::Node) -> usize;
    fn neighbors(&self, u: &Self::Node) -> impl Iterator<Item = Self::Node> + '_;
    fn heuristic(&self, u: &Self::Node) -> usize;

    fn a_star(&self, start: Self::Node, goal: Self::Node) -> Vec<Self::Node> {
        let mut open_set = HashSet::<Self::Node>::new();
        let mut open = BinaryHeap::<NodeScore<Self::Node>>::new();
        open_set.insert(start);
        open.push(NodeScore {
            score: 0,
            node: start,
        });
        let mut prev = HashMap::<Self::Node, Self::Node>::new();
        let mut g_map = HashMap::<Self::Node, usize>::new();
        g_map.insert(start, 0);

        while let Some(curr) = open.pop() {
            open_set.remove(&curr.node);
            if curr.node == goal {
                let mut path = vec![goal];
                let mut u = &goal;
                while let Some(v) = prev.get(u) {
                    path.insert(0, *v);
                    u = v;
                }
                return path;
            }
            self.neighbors(&curr.node).for_each(|neighbor| {
                let g_score = g_map
                    .get(&curr.node)
                    .map(|s| s + self.weight(&curr.node, &neighbor))
                    .unwrap_or(usize::MAX);
                if &g_score < g_map.get(&neighbor).unwrap_or(&usize::MAX) {
                    prev.insert(neighbor, curr.node);
                    g_map.insert(neighbor, g_score);
                    if !open_set.contains(&neighbor) {
                        open_set.insert(neighbor);
                        open.push(NodeScore {
                            score: g_score + self.heuristic(&neighbor),
                            node: neighbor,
                        });
                    }
                }
            });
        }
        vec![]
    }
}

impl AStar for Map {
    type Node = Ix2;

    fn weight(&self, _: &Self::Node, _: &Self::Node) -> usize {
        1
    }

    fn neighbors(&self, u: &Self::Node) -> impl Iterator<Item = Ix2> + '_ {
        let height = self.map[*u];

        let (y, x) = u.into_pattern();
        let neighbors = [Ix2(y - 1, x), Ix2(y + 1, x), Ix2(y, x - 1), Ix2(y, x + 1)];

        neighbors
            .into_iter()
            .filter(move |ix| self.map[*ix] <= height + 1)
    }

    fn heuristic(&self, u: &Self::Node) -> usize {
        let (ty, tx) = self.goal.into_pattern();
        let (y, x) = u.into_pattern();
        (ty as isize - y as isize).unsigned_abs() + (tx as isize - x as isize).unsigned_abs()
    }
}

impl std::str::FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ncols = s.find('\n').unwrap();
        let nrows = s.len() / ncols;
        let s = s.replace('\n', "");
        let mut map = Array2::from_elem((nrows + 2, ncols + 2), u8::MAX);
        let idx_start = s.find('S').unwrap();
        let idx_goal = s.find('E').unwrap();
        let start = Ix2(1 + idx_start / ncols, 1 + idx_start % ncols);
        let goal = Ix2(1 + idx_goal / ncols, 1 + idx_goal % ncols);
        let a = Array::from_iter(s.chars().filter_map(|c| match c {
            'S' => Some(0),
            'E' => Some(25),
            'a'..='z' => Some(c as u8 - b'a'),
            _ => None,
        }))
        .into_shape((nrows, ncols))
        .unwrap();
        map.slice_mut(s![1..-1, 1..-1]).assign(&a);
        Ok(Map { map, start, goal })
    }
}

#[derive(Eq)]
struct NodeScore<T> {
    score: usize,
    node: T,
}

impl<T: Eq> Ord for NodeScore<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl<T: Eq> PartialOrd for NodeScore<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq> PartialEq for NodeScore<T> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
