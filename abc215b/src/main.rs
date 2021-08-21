// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u128,
    }
    println!("{}", solve(n));
}

fn solve(n: u128) -> u32 {
    if n < 2 {
        return 0;
    }

    let mut count = 0;
    let mut answer = 2_u128;
    while answer <= n {
        answer = answer * 2;
        count = count + 1;
    }
    return count;
}
