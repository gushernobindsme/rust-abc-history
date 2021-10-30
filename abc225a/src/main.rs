// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let list = s.iter().collect::<Vec<_>>();

    // 雑に全パターンを網羅
    let mut patterns: Vec<String> = Vec::new();
    patterns.push([list[0].to_string(),list[1].to_string(),list[2].to_string()].join(""));
    patterns.push([list[0].to_string(),list[2].to_string(),list[1].to_string()].join(""));
    patterns.push([list[1].to_string(),list[0].to_string(),list[2].to_string()].join(""));
    patterns.push([list[1].to_string(),list[2].to_string(),list[0].to_string()].join(""));
    patterns.push([list[2].to_string(),list[0].to_string(),list[1].to_string()].join(""));
    patterns.push([list[2].to_string(),list[1].to_string(),list[0].to_string()].join(""));

    // 重複排除
    patterns.sort();
    patterns.dedup();

    // 件数をカウント
    println!("{}", patterns.len());
}
