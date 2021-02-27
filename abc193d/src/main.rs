// -*- coding:utf-8-unix -*-

use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        k: u32,  // 1〜9のカードが何セットあるか
        s: String, // 高橋くん
        t: String, // 青木くん
    }

    // 山札にカードをセット
    let mut deck = BTreeMap::new(); // (カードの種類, 残り枚数)
    for i in 1..=9 {
        deck.insert(i, k);
    }

    // 高橋くんにカードを配る
    let mut t_score = 0;
    let t_cards = s
        .chars()
        .filter(|v| v.to_string() != "#")
        .collect::<Vec<_>>();
    for card in t_cards {
        let suit = card.to_string().parse::<i32>().unwrap();
        if let Some(quantity) = deck.get(&suit) {
            deck.insert(suit, quantity - 1);
            t_score += suit;
        }
    }
    // 青木くんにカードを配る
    let mut a_score = 0;
    let a_cards = t
        .chars()
        .filter(|v| v.to_string() != "#")
        .collect::<Vec<_>>();
    for card in a_cards {
        let suit = card.to_string().parse::<i32>().unwrap();
        if let Some(quantity) = deck.get(&suit) {
            deck.insert(suit, quantity - 1);
            a_score += suit;
        }
    }

    // 山札から残りのカードを取り出す
    let mut remain_cards = Vec::new();
    for (suit, quantity) in deck {
        if quantity == 0 {
            continue;
        }
        for _ in 1..=quantity {
            remain_cards.push(suit);
        }
    }

    // 残りカードの順列を取得
    let combinations = remain_cards
        .iter()
        .permutations(2)
        .map(|v| (v[0], v[1]))
        .collect::<Vec<_>>();

    // 高橋くんが勝利するケースを求める
    let mut count = 0;
    for combination in combinations.clone() {
        let (t_last_card, a_last_card) = combination;
        if (t_score + t_last_card) > (a_score + a_last_card) {
            count += 1;
        }
    }
    println!("{}", (count as f64 / combinations.len() as f64));
}
