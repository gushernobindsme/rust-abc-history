// -*- coding:utf-8-unix -*-

use itertools::Itertools;
use num::integer::*;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    // これだと TLE になる
    let answer = (a..=b)
        .combinations(2)
        .map(|combination| {
            let x = combination[0];
            let y = combination[1];
            gcd(x, y)
        })
        .max();
    println!("{}", answer.unwrap());
}
