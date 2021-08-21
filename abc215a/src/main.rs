// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    println!("{}", if s.eq("Hello,World!") { "AC" } else { "WA" } );
}
