// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut trees: [i32; n],
    }
    let mut ans = 0;
    for tree in trees {
        if tree <= 10 {
            continue;
        }
        ans = ans + (tree - 10);
    }
    println!("{}", ans);
}
