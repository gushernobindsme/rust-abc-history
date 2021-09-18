// -*- coding:utf-8-unix -*-

use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        (x, y): (usize, usize), // 高橋くんは X 個以上のたこ焼きと Y 個以上のたい焼きをたべたい
        mut rows: [(usize, usize); n], // お弁当のリスト。 (たこ焼きの数, たい焼きの数) のタプルを持つ
    }

    // この問題は動的計画法で解ける
    // 多重配列の中には購入した弁当の個数を格納する
    let mut dp = vec![vec![vec![None; y + 1]; x + 1]; n + 1];
    dp[0][0][0] = Some(0);

    // n:弁当の個数、x:求めるたこ焼きの数、y:求めるたい焼きの数で全探索する
    // いずれの値も十分に小さいため、計算量 O(nxy) で解くことができる
    // より少ない回数で条件を満たせるパターンが見つければ、都度、最小値に入れ替えていく
    for i in 0..n {
        for j in 0..=x {
            for k in 0..=y {
                if let Some(v) = dp[i][j][k] {
                    // 高橋くんが i 種類目の弁当を買わない場合：
                    // - 状態を (i, j, k) に遷移させる
                    // - 購入した弁当の個数は増えない
                    if let Some(v1) = dp[i + 1][j][k] {
                        dp[i + 1][j][k] = Some(min(v, v1));
                    } else {
                        dp[i + 1][j][k] = Some(v);
                    }
                    // 高橋くんが i 種類目の弁当を買う場合：
                    // - a 個のたこ焼きと b 個のたい焼きが手に入る
                    // - 状態を (i, min(j + a, x), min(k + b, y)) に遷移させる
                    // - 購入した弁当の個数を +1 する
                    let (a, b) = rows[i];
                    // 配列のサイズを超えてしまわないよう、min をとる
                    let ax = min(j + a, x);
                    let by = min(k + b, y);
                    if let Some(v1) = dp[i + 1][ax][by] {
                        dp[i + 1][ax][by] = Some(min(v + 1, v1));
                    } else {
                        dp[i + 1][ax][by] = Some(v + 1);
                    }
                }
            }
        }
    }

    // 最終的に見つかった最も小さい購入回数を採用する
    if let Some(v) = dp[n][x][y] {
        println!("{}", v);
    } else {
        println!("-1");
    }
}
