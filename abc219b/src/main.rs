// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s1: String,
    }
    input! {
        s2: String,
    }
    input! {
        s3: String,
    }
    input! {
        mut t: Chars,
    }
    let result = t
        .iter()
        .map(|v| {
            if v.to_string() == "1" {
                s1.to_string()
            } else if v.to_string() == "2" {
                s2.to_string()
            } else if v.to_string() == "3" {
                s3.to_string()
            } else {
                panic!()
            }
        })
        .collect::<Vec<_>>();

    println!("{}", result.join(""));
}
