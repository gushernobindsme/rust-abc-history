// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut chars = s.chars().collect::<Vec<char>>();
    let first = chars.remove(0);
    chars.push(first);

    let mut answer = String::new();
    chars.into_iter().for_each(|c| answer.push(c));
    println!("{}", answer);
}
