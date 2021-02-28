// -*- coding:utf-8-unix -*-

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        k: u32,  // 1〜9のカードが何セットあるか
        s: String, // 高橋くんの手札
        t: String, // 青木くんの手札
    }
    let mut deck = [k; 9];

    // 表向きのカードをリストに詰める
    let s = s.bytes().take(4).map(|x| (x - b'0') as usize).collect_vec();
    let t = t.bytes().take(4).map(|x| (x - b'0') as usize).collect_vec();

    // 山札から使用済みのカードを取り除く
    for i in s.clone() {
        deck[i - 1] -= 1;
    }
    for i in t.clone() {
        deck[i - 1] -= 1;
    }

    let mut answer = 0_f64;

    // 「高橋くんが持っている裏向きのカード」と「青木くんが持っている裏向きのカード」が異なる場合
    for i in 1..=9 {
        // 山札の残り枚数が 0 のカードの場合、何もしない
        if deck[i - 1] == 0 {
            continue;
        }
        for j in 1..=9 {
            // カードの種類が同じ場合 or 山札の残り枚数が 0 のカードの場合、何もしない
            if i == j || deck[j - 1] == 0 {
                continue;
            }
            // 「高橋くんの合計点数」が「青木くんの合計点数」よりも高い場合、
            // 山札に残っているカードの組み合わせを answer に合算する
            if score(s.iter().copied().chain(vec![i]).collect())
                > score(t.iter().copied().chain(vec![j]).collect())
            {
                answer += deck[i - 1] as f64 * deck[j - 1] as f64
            }
        }
    }

    // 「高橋くんが持っている裏向きのカード」と「青木くんが持っている裏向きのカード」が同じ場合
    for i in 1..=9 {
        // 山札に同じ種類のカードが存在しない場合、何もしない
        if deck[i - 1] < 2 {
            continue;
        }
        // 「高橋くんの合計点数」が「青木くんの合計点数」よりも高い場合、
        // 山札に残っているカードの組み合わせを answer に合算する
        if score(s.iter().copied().chain(vec![i]).collect())
            > score(t.iter().copied().chain(vec![i]).collect())
        {
            answer += deck[i - 1] as f64 * (deck[i - 1] - 1) as f64
        }
    }

    // 「高橋くんが勝つ確率」は、「高橋くんの勝つ配り方の数」 / 「全ての配り方の数」で求められる
    // 「全ての配り方の数」を求めるには、山札に残ったカードから残り二枚を総当たりした順列を求めれば良い
    // これは `山札に残ったカードの枚数 P 2` と表せるため、
    // `(9K - 8) P 2` となり、 `(9 * k - 8) * (9 * k - 8 - 1)` が分母になる
    let total = 9 * k - 8;
    println!("{}", (answer as f64 / total as f64 / (total - 1) as f64));
}

/// 手札の配列を元に得点を計算する
fn score(cards: Vec<usize>) -> i32 {
    // どのカードを何枚持っているかを count に格納する
    let mut count = [0; 9];
    for card in cards {
        count[card - 1] += 1;
    }

    let mut score = 0;
    // 1 〜 9 までの数字の総和を求める
    for i in 1..=9 {
        // カードが 0 枚なら 10 の 0 乗、
        // カードが 1 枚なら 10 の 1 乗、
        // カードが 2 枚なら 10 の 2 乗、……という計算を行う
        score += i as i32 * 10_i32.pow(count[i - 1]);
    }
    score
}
