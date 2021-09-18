// -*- coding:utf-8-unix -*-

use proconio::input;

enum Rank {
    Beginner,
    Intermediate,
    Advanced,
    Expert
}

fn main() {
    input! {
        x: usize,
    }
    let rank = if x < 40 {
        Rank::Beginner
    } else if 40 <= x && x < 70 {
        Rank::Intermediate
    } else if 70 <= x && x < 90 {
        Rank::Advanced
    } else {
        Rank::Expert
    };
    match rank {
        Rank::Beginner => { println!("{}", 40 - x) },
        Rank::Intermediate => { println!("{}", 70 - x) },
        Rank::Advanced => { println!("{}", 90 - x) },
        Rank::Expert => { println!("expert") }
    };
}
