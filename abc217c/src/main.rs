// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        mut p: [usize; n],
    }
    let mut q = HashMap::new(); // key が index、value が q_i
    for i in 0..n {
        let index = p[i];
        q.insert(index, i + 1);
    }

    // index 順に並び替える
    let mut sorted = q.into_iter().collect::<Vec<_>>();
    sorted.sort_by_key(|v| v.0);

    // value を取り出して空白区切りで出力する
    let answer = sorted
        .into_iter()
        .map(|v| v.1.to_string())
        .collect::<Vec<_>>();
    println!("{}", answer.join(" "));
}
