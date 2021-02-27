// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut rows: [(i32, i32, i32); n],
    }
    let mut amounts = Vec::new();
    for row in rows {
        // a: 高橋くんの現在地からお店までの距離（分）
        // p: スヌケマシンの販売価格（円）
        // x: スヌケマシンの在庫（台）
        let (a, p, x) = row;
        if x - a > 0 {
            amounts.push(p);
        }
    }

    if let Some(amount) = amounts.iter().min() {
        println!("{}", amount);
    } else {
        println!("{}", -1);
    }
}
