// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        x: Chars,
        n: usize,
        mut s: [Chars; n],
    }
    // 高橋君の定めたアルファベット順の変換表を作る
    let mut alphabet_to_number = HashMap::new();
    let mut number_to_alphabet = HashMap::new();
    x.iter().enumerate().for_each(|(i, v)| {
        alphabet_to_number.insert(*v, format!("{:02}", i));
        number_to_alphabet.insert(format!("{:02}", i), *v);
    });

    // 変換表をもとにただの数値に置き換える
    let mut results = s
        .iter()
        .map(|row| {
            row.iter()
                .map(|v| alphabet_to_number.get(v).unwrap())
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // 辞書順にソートする
    results.sort();

    // ソート結果を元々の文字列に置き換える
    for result in results {
        let original = result
            .iter()
            .map(|v| number_to_alphabet.get(v).unwrap().to_string())
            .collect::<Vec<_>>();
        println!("{}", original.join(""));
    }
}
