// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u128,
    }
    // n の桁数を調べる
    let digits = n.to_string().len();

    let mut target_digits = (digits / 2) + 1;
    // x の十進表記が偶数桁の場合は半分の桁で走査
    if digits % 2 == 0 {
        target_digits = digits / 2;
    }

    // 提示された数字の半分の桁について、 min - max のリストを作る
    let max = 10_u128.pow(target_digits as u32);
    let pattern1 = (1..max).collect::<Vec<_>>();
    let pattern2 = (1..max).collect::<Vec<_>>();

    // 前半と後半が文字列として等しくなるパターンを用意する
    let mut ans = 0;
    for (i, j) in pattern1.iter().zip(pattern2.iter()) {
        let x = (i.to_string() + &*j.to_string()).parse::<u128>().unwrap();
        // 1 以上 N 以下の整数 x の個数をカウント
        if 1_u128 <= x && x <= n {
            ans += 1;
        }
    }

    println!("{}", ans);
}
