// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut names: [(String, String); n],
    }
    let mut answer = "No";

    let mut list = Vec::new();
    for (s, t) in names {
        let name = s + "_" + &*t;
        if list.contains(&name) {
            answer = "Yes";
            break;
        }
        list.push(name);
    }

    println!("{}", answer);
}
