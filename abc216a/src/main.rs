// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        xy: String,
    }
    let v = xy.split(".").collect::<Vec<_>>();
    let x: i32 = v[0].parse().unwrap();
    let y: i32 = v[1].parse().unwrap();
    let category = if 0 <= y && y <= 2 {
        "-"
    } else if 7 <= y && y <= 9 {
        "+"
    } else {
        ""
    };
    println!("{}{}", x, category);
}
