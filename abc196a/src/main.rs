// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (a, b): (i32, i32),
        (c, d): (i32, i32),
    }
    let x = b;
    let y = c;
    println!("{}", x - y);
}
