// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut routes: [(usize, usize); m],
    }
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for (a, b) in routes {
        graph[a].push(b);
    }

    let mut ans = 0;
    for i in 1..=n {
        ans += bfs(&graph, i);
    }
    println!("{}", ans);
}

/// graph 上の 1 頂点 s を始点として、幅優先探索を実施
fn bfs(graph: &Vec<Vec<usize>>, s: usize) -> usize {
    let n = graph.len(); // 頂点数
    let mut dist = vec![-1; n]; // 全ての頂点を「未訪問」に初期化
    let mut count = 0;
    let mut queue = VecDeque::new();

    dist[s] = 0; // スタート地点を「訪問済」に初期化する
    queue.push_front(s); // スタート地点をキューに登録する

    while !queue.is_empty() {
        let v = queue.pop_back().unwrap(); // キューから先頭頂点を取り出す

        // v からたどれる頂点をすべて調べる
        for &x in graph[v].iter() {
            // 「訪問済」の頂点は探索しない
            if dist[x] != -1 {
                continue;
            }
            dist[x] = dist[v] + 1; // 「未訪問」を「訪問済」に更新
            queue.push_front(x); // 次の街をキューに登録する
        }
        count += 1;
    }
    count
}
