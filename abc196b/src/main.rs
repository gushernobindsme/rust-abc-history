// -*- coding:utf-8-unix -*-

use proconio::input;
use regex::Regex;

fn main() {
    input! {
       x: String,
    }
    let re = Regex::new(r"\..?[0-9]*$").unwrap();
    let ans = re.replace_all(&x, "");
    println!("{}", ans);
}
