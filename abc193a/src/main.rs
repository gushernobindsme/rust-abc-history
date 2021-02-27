// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (a, b): (f64, f64),
    }
    println!("{}", (1_f64 - (b / a)) * 100_f64);
}
