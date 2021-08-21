// -*- coding:utf-8-unix -*-

use num::integer::gcd;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    // 1 以上 m 以下の整数 k を探索
    let mut target: HashSet<usize> = (1..m).into_iter().collect();
    for ai in a {
        let mut gcd_set = HashSet::new();
        for i in target.clone() {
            let gcd = gcd(ai, i);
            if gcd == 1 {
                gcd_set.insert(i);
            }
        }
        // 直積を求める
        target = target
            .intersection(&gcd_set)
            .map(|x| *x)
            .collect::<HashSet<usize>>();
    }

    // 一行目に整数の数を出力
    println!("{}", target.len());

    // 続く x 行に答えとなる整数を小さい方から順に一行ずつ出力
    let mut answers = target.into_iter().collect::<Vec<_>>();
    answers.sort();
    for answer in answers {
        println!("{}", answer);
    }
}
