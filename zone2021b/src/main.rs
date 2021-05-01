// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        // N: 遮蔽物の数
        // D: タワーから UFO までの距離
        // H: UFO の高さ
        (n, d, h): (usize, usize, usize),
        // (d: 遮蔽物の距離, h: 遮蔽物の高さ)
        mut rows: [(usize, usize); n],
    }

    // UFO と遮蔽物の上端を結ぶ直線の傾きは (H-h) / (D-d) で求められる。
    // （タワーを登らずとも UFO に電波を届けることができる場合もあるため）
    // 登らなければならない高さは max( 0, h - d * (H-h) / (D-d) ) で求められる。
    // 遮蔽物が複数ある場合は、これらの最大値が答えとなる。
    let mut goal = 0.;
    for row in rows {
        let distance = row.0  as f64;
        let height = row.1 as f64;
        let point = height - distance * (h as f64 - height) / (d as f64 - distance);
        if goal < point {
            goal = point;
        }
    }
    println!("{}", goal);
}
