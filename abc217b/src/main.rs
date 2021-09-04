// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        s1: String
    }
    input! {
        s2: String
    }
    input! {
        s3: String
    }
    let mut list = vec![
        "ABC".to_string(),
        "ARC".to_string(),
        "AGC".to_string(),
        "AHC".to_string(),
    ]
    .into_iter()
    .collect::<HashSet<_>>();
    list.remove(&s1);
    list.remove(&s2);
    list.remove(&s3);

    let answer = list.iter().map(|v| v.to_string()).collect::<Vec<_>>();
    println!("{}", answer[0]);
}
