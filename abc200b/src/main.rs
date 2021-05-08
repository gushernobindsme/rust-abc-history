// -*- coding:utf-8-unix -*-

use proconio::input;
use num_integer::Integer;

fn main() {
    input! {
        // n: 整数
        // k: 操作を行う回数
        (n, k): (usize, usize),
    }
    let mut answer = n;
    for _i in 0..k {
        if answer.is_multiple_of(&200) {
            // 200 の倍数の場合、 200 で割る
            answer = answer / 200;
        } else {
            // それ以外の場合、「+"200"」する
            answer = answer * 1000 + 200;
        }
    }
    println!("{}", answer);
}
