// -*- coding:utf-8-unix -*-

use petgraph::{Graph, Undirected};
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut paths: [(u32, u32); n-1],
    }

    // グラフを生成
    let mut graph = Graph::<(), (), Undirected>::new_undirected();
    graph.extend_with_edges(&paths);

    let mut answer = false;
    for node_index in graph.node_indices() {
        let neighbors = graph.neighbors_undirected(node_index).count();
        // 隣接する要素の数が n-1 の枝があればスター型のグラフといえる
        if neighbors == n - 1 {
            answer = true;
            continue;
        }
    }

    println!("{}", if answer { "Yes" } else { "No" });
}
