// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    println!("{}", if s < t { "Yes" } else { "No" });
}
