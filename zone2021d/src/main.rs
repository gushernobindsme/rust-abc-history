// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        s: String,
    }
    // T を空文字列とする
    let mut t: VecDeque<String> = VecDeque::new();

    // 現在反転しているかの変数を持たせる。
    // 反転していない場合は末尾に追加する
    // 反転している場合は先頭に追加する
    let mut is_rev = false;

    // i = 1,2,...,S について処理を行う
    for c in s.chars() {
        if c.to_string() == "R" {
            is_rev = !is_rev;
        } else if t.is_empty() {
            t.push_back(c.to_string());
        } else if is_rev {
            if t.front().unwrap().to_string() == c.to_string() {
                // 同じ文字が連続する場合、先頭から削除する
                t.pop_front();
            } else {
                // 同じ文字ではない場合、先頭に追加する
                t.push_front(c.to_string());
            }
        } else {
            if t.back().unwrap().to_string() == c.to_string() {
                // 同じ文字が連続する場合、末尾から削除する
                t.pop_back();
            } else {
                // 同じ文字ではない場合、末尾に追加する
                t.push_back(c.to_string());
            }
        }
    }
    let mut vec = t.iter().map(|v| v.to_string()).collect::<Vec<_>>();

    // is_rev が true で終了した場合は全体を反転させる
    if is_rev {
        vec.reverse();
    }
    let answer = vec.join("");
    println!("{}", answer);
}
