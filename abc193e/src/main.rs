// -*- coding:utf-8-unix -*-

use num::Integer;
use proconio::input;

fn main() {
    input! {
        t: usize,
        cases: [(i128, i128, i128, i128); t],
    }

    for case in cases {
        // x: A 街から B 街に着くまでの移動時間
        // y: 駅で停車している時間
        // p: 高橋くんが寝ている時間
        // q: 高橋くんが起きている時間
        let (x, y, p, q) = case;
        // c: 電車が行って戻ってくる周期
        let c = (x + y) * 2;
        // d: 高橋くんが寝て目覚める周期
        let d = p + q;
        let mut ans = std::i128::MAX;

        for a in x..(x + y) {
            for b in p..(p + q) {
                // t = a (mod c)  …… c で割ると a 余る
                // t = b (mod d)  …… d で割ると b 余る
                // を満たす整の非負の数を求めれば良い
                if let Some((r, _m)) = crt(a, c, b, d) {
                    assert_ne!(r, 0);
                    ans = ans.min(r);
                }
            }
        }
        if ans >= std::i128::MAX {
            println!("infinity");
        } else {
            println!("{}", ans);
        }
    }
}

/// m1 で割ると b1 余り、m2 で割ると b2 余る数を求める（中国剰余定理）。
/// 答えは `x = y (mod z)` の形で書けることが知られている。
/// 答えが存在する場合、 Some(y, z) を返す。
fn crt(b1: i128, m1: i128, b2: i128, m2: i128) -> Option<(i128, i128)> {
    let (d, p, q) = ext_gcd(m1, m2);
    assert_eq!(d, p * m1 + q * m2);
    if (b2 - b1) % d != 0 {
        return None;
    }
    let m = m1.lcm(&m2);
    let t = (b2 - b1) / d * p.rem_euclid(m2 / d);
    let r = (b1 + m1 * t).rem_euclid(m);
    Some((r, m))
}

/// ユークリッドの互助法により最小公約数を求める。
fn ext_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, y, x) = ext_gcd(b.rem_euclid(a), a);
        (g, x - (b / a) * y, y)
    }
}
