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
    for i in 1..=9 {
        if deck[i - 1] == 0 {
            continue;
        }
        for j in 1..=9 {
            if i == j || deck[j - 1] == 0 {
                continue;
            }
            if score(s.iter().copied().chain(vec![i]).collect())
                > score(t.iter().copied().chain(vec![j]).collect())
            {
                answer += deck[i - 1] as f64 * deck[j - 1] as f64
            }
        }
    }

    for i in 1..=9 {
        if deck[i - 1] < 2 {
            continue;
        }
        if score(s.iter().copied().chain(vec![i]).collect())
            > score(t.iter().copied().chain(vec![i]).collect())
        {
            answer += deck[i - 1] as f64 * (deck[i - 1] - 1) as f64
        }
    }

    let total = 9 * k - 8;
    println!("{}", (answer as f64 / total as f64 / (total - 1) as f64));
}

/// 手札の配列を元に得点を計算する
fn score(cards: Vec<usize>) -> i32 {
    let mut count = [0; 9];
    for card in cards {
        count[card - 1] += 1;
    }

    let mut score = 0;
    for i in 1..=9 {
        score += i as i32 * 10_i32.pow(count[i - 1]);
    }
    score
}
