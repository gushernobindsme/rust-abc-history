// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, // 整数の数
        mut a: [u32; n], // 整数
    }
    // 「Ai - Aj が 200 の倍数である」とは、
    // 「Ai を 200 で割ったあまりと Aj を 200 で割ったあまりが一致する」と言い換えられる

    // 200 で割ったあまりが一致するデータが何件あるかを調べる
    let mut map = HashMap::new();
    for x in a {
        *map.entry(x % 200).or_insert(0) += 1;
    }
    // 「位置が異なる2つの要素を選ぶ組み合わせ」が何通りあるかを調べる
    // つまり、 x * (x - 1) / 2 を求めれば良い
    let answer = map.iter().map(|(_, &x)| x * (x - 1) / 2).sum::<usize>();
    println!("{}", answer);

    // n を数が多いので馬鹿正直に解こうとすると TLE になる

    // for i in 0..n {
    //     for j in i + 1..n {
    //         let left = a[i];
    //         let right = a[j];
    //         // 0 は 200 の倍数とみなす
    //         if left == right {
    //             answer = answer + 1;
    //             continue;
    //         }
    //         if left < right {
    //             if (right - left).is_multiple_of(&200_u128) {
    //                 answer = answer + 1;
    //             }
    //         } else {
    //             if (left - right).is_multiple_of(&200_u128) {
    //                 answer = answer + 1;
    //             }
    //         }
    //     }
    // }
    // println!("{}", answer);
}
