// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    println!("{}", (n + (100 - 1)) / 100);
}
