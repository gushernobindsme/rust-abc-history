// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h: usize, // 縦
        w: usize, // 横
        y: usize, // 現在地X座標
        x: usize, // 現在地Y座標（ムカつくので逆にします
        mut rows: [String; h],  // 部屋の状態
    }
    // `.` と `#` を埋めたマップをつくる
    let mut floor: Vec<Vec<char>> = Vec::new();
    for row in rows {
        floor.push(row.chars().collect::<Vec<_>>())
    }

    // マス (x, y) に障害物は置かれていない
    let mut answer = 1;

    // ←
    for i in (1..x).rev() {
        if floor[y - 1][i - 1].to_string() == "." {
            answer += 1;
        } else {
            break;
        }
    }
    // →
    for i in x + 1..=w {
        if floor[y - 1][i - 1].to_string() == "." {
            answer += 1;
        } else {
            break;
        }
    }
    // ↑
    for j in (1..y).rev() {
        if floor[j - 1][x - 1].to_string() == "." {
            answer += 1;
        } else {
            break;
        }
    }
    // ↓
    for j in y + 1..=h {
        if floor[j - 1][x - 1].to_string() == "." {
            answer += 1;
        } else {
            break;
        }
    }
    println!("{}", answer);
}
