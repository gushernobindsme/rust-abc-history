// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let answer = s.matches("ZONe").count();
    println!("{}", answer);
}
