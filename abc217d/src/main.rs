// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        l: usize, // 木材の長さ
        q: usize, // クエリの回数
        mut cx: [(usize, usize); q], // 具体的なコマンドのセット
    }
    let mut set = BTreeSet::new();
    set.insert(l);
    let mut answers = Vec::new();

    for i in 0..q {
        let command = cx[i].0;
        let point = cx[i].1;
        if command == 1 {
            // 線 x_i がある地点で木材を二つに切る
            set.insert(point);
        } else if command == 2 {
            // 線 x_i を含む木材を選び、その長さを出力する
            let next_option = set.range(point..).next(); // 指定した箇所以上を探したときの最初の要素
            let prev_option = set.range(..=point).next_back(); // 指定した箇所以下を探したときの最後の要素
            match (next_option, prev_option) {
                (Some(next), Some(prev)) => answers.push(*next - *prev), // 中間にあたる場合
                (None, Some(prev)) => answers.push(*prev), // 一度も切られていない場合
                (Some(next), None) => answers.push(*next), // 一度も切られていない場合
                _ => panic!(),
            };
        } else {
            panic!();
        }
    }

    for answer in answers {
        println!("{}", answer);
    }
}
