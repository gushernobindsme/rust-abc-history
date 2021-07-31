// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }
    if b == 0 {
        println!("Gold");
    } else if a == 0 {
        println!("Silver");
    } else {
        println!("Alloy");
    }
}
