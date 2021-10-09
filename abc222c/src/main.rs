// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

enum Suite {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum Result {
    Win,
    Lose,
    Draw,
}

fn main() {
    input! {
        n: usize, // 試合数。じゃんけん大会は 2n 人で行う
        m: usize, // ラウンド数
    }

    // 高橋くんは 2n 人のプレイヤが、各ラウンドで出す手を予知できる
    let mut oracles = Vec::new();
    for _ in 0..2 * n {
        input! {
            oracle: Chars, // 高橋くんの予知した結果
        }
        oracles.push(oracle);
    }

    // プレイヤのスコアを初期化
    // (index, score)
    let mut rank = (0..2 * n).into_iter().map(|i| (i ,0)).collect::<Vec<_>>();

    // 試合開始
    for j in 0..m {
        for i in 0..n {
            let player1 = rank[2 * i].0;
            let player2 = rank[2 * i + 1].0;

            let left = char_to_suite(oracles[player1][j]);
            let right = char_to_suite(oracles[player2][j]);
            let (left_result, right_result) = battle(left, right);
            if left_result == Result::Win {
                rank[2 * i].1 += 1;
            }
            if right_result == Result::Win {
                rank[2 * i + 1].1 += 1;
            }
        }
        rank.sort_by(|a, b| {
            if a.1 == b.1 {
                // スコアが同じなら、index が若い人を優先する
                a.0.cmp(&b.0)
            } else {
                // スコアの高い順に並べる
                b.1.cmp(&a.1)
            }
        });
    }
    for (index, _) in rank {
        println!("{}", index + 1);
    }
}

fn char_to_suite(value: char) -> Suite {
    if value == 'G' {
        Suite::Rock
    } else if value == 'C' {
        Suite::Scissors
    } else if value == 'P' {
        Suite::Paper
    } else {
        panic!()
    }
}

fn battle(left: Suite, right: Suite) -> (Result, Result) {
    match (left, right) {
        (Suite::Rock, Suite::Paper) => (Result::Lose, Result::Win),
        (Suite::Rock, Suite::Scissors) => (Result::Win, Result::Lose),
        (Suite::Paper, Suite::Rock) => (Result::Win, Result::Lose),
        (Suite::Paper, Suite::Scissors) => (Result::Lose, Result::Win),
        (Suite::Scissors, Suite::Rock) => (Result::Lose, Result::Win),
        (Suite::Scissors, Suite::Paper) => (Result::Win, Result::Lose),
        _ => (Result::Draw, Result::Draw),
    }
}
