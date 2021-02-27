// -*- coding:utf-8-unix -*-
use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut d = vec![];
    for b in 2.. {
        // 乗数が n を超えたらループを終了する
        if 2u64.pow(b) > n {
            break;
        }
        // 2 以上の数字 a の b 乗が n 以下になるものを取り出す
        // a の b 乗をリストに詰める
        d.extend((2u64..).take_while(|a| a.pow(b) <= n).map(|a| a.pow(b)));
    }
    // 重複を排除
    d.sort();
    d.dedup();
    let ans = n as usize - d.len();
    println!("{}", ans);
}
