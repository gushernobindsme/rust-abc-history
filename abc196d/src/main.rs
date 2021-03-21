// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h: usize, // 部屋の縦
        w: usize, // 部屋の横
        a: usize, // 2メートル×1メートルの畳の枚数
        b: usize, // 1メートル×1メートルの畳の枚数
    }
    let mut ans = 0;
    let mut dfs = vec![(0, 0, a, b, 0)];

    // 左上のマスから順に、半畳、右にはみ出す畳、下にはみ出す畳を敷いていく
    // H * W <= 16 と制約が十分に小さいので、敷き詰め方を全探索する（深さ優先探索）
    while let Some((x, y, a, b, s)) = dfs.pop() {
        if x == h {
            // 畳を敷き終えた場合
            ans += 1;
        } else if y == w { // 縦方向の最終マスまで来た場合
            // 一つ右にずれて、一番上に戻る
            dfs.push((x + 1, 0, a, b, s));
        } else if s >> y & 1 == 1 { // すでに畳を敷いてある場合
            dfs.push((x, y + 1, a, b, s ^ (1 << y)));
        } else {
            if b > 0 { // 半畳の畳を使える場合
                // 一つ下にずれる
                dfs.push((x, y + 1, a, b - 1, s));
            }
            if a > 0 { // 長方形の畳を使える場合
                // 長方形の畳を下にはみ出す形で敷けないか試してみる
                if y + 1 < w && s >> (y + 1) & 1 == 0 {
                    dfs.push((x, y + 2, a - 1, b, s));
                }
                // 長方形の畳を右にはみ出す形で敷けないか試してみる
                if x + 1 < h {
                    dfs.push((x, y + 1, a - 1, b, s | (1 << y)));
                }
            }
        }
    };
    println!("{}", ans);
}
