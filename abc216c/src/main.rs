// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut answer = Vec::new();
    let mut target = n;
    while target != 0 {
       if target % 2 == 0 {
           // 偶数ならば B の魔法を使う
           target = target / 2;
           answer.push("B");
       } else {
           // 奇数ならば A の魔法を使う
           target = target - 1;
           answer.push("A");
       }
    };
    answer.reverse();
    println!("{}", answer.iter().cloned().collect::<String>());
}
