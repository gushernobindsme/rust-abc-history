// -*- coding:utf-8-unix -*-

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        k: usize,
    }

    // ベースとなる文字列を辞書順にソート
    let mut chars = s.into_iter().collect::<Vec<_>>();
    chars.sort();

    // とりうる組み合わせの index をつくる
    let index_list = (0..chars.len())
        .permutations(chars.len())
        .into_iter()
        .collect::<Vec<_>>();

    // とりうる値の組み合わせを作る
    let mut patterns = Vec::new();
    for indexes in index_list {
        let mut pattern = Vec::new();
        for index in indexes {
            pattern.push(chars[index]);
        }
        patterns.push(pattern);
    }
    patterns.sort();
    patterns.dedup();

    // k 番目の値を出力
    let ans: String = patterns[k - 1].iter().collect();
    println!("{}", ans);
}
