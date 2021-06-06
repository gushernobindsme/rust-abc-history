// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (x, y): (i32, i32),
    }
    let ans = solve(x, y).unwrap();
    println!("{}", ans);
}

fn solve(x: i32, y: i32) -> Option<i32> {
    match (x, y) {
        (0, 0) => Some(0),
        (0, 1) => Some(2),
        (0, 2) => Some(1),
        (1, 0) => Some(2),
        (1, 1) => Some(1),
        (1, 2) => Some(0),
        (2, 0) => Some(1),
        (2, 1) => Some(0),
        (2, 2) => Some(2),
        _ => None,
    }
}
