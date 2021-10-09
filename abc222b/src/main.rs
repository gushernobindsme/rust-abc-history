// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize, // 学生の人数
        p: usize, // 合格ライン
        mut scores: [usize; n], // 点数のリスト
    }
    let answer = scores.into_iter().filter(|a| a < &p).count();
    println!("{}", answer);
}
