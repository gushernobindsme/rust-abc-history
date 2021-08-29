// -*- coding:utf-8-unix -*-

use proconio::input;

use itertools::Itertools;
use petgraph::{algo::is_cyclic_directed, graph::DiGraph};

fn main() {
    input! {
        _: usize,
        m: usize,
    }
    let mut paths: Vec<(usize, usize)> = Vec::new();
    for _ in 0..m {
        input! {
            k: usize,
            bs: [usize; k],
        }
        for w in bs.into_iter().tuple_windows() {
            paths.push(w);
        }
    }
    let g = DiGraph::<(), (), usize>::from_edges(&paths);
    if is_cyclic_directed(&g) {
        println!("No");
    } else {
        println!("Yes");
    }
}
