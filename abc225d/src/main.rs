// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, // 電車の数
        q: usize, // クエリの数
    }

    let mut front: Vec<_> = (0..n + 1).map(|_| None).collect(); // front[i] := 電車iの前部に連結している電車。ないならNone
    let mut back: Vec<_> = (0..n + 1).map(|_| None).collect(); // back[i] := 電車iの後部に連結している電車。ないならNone
    for _ in 0..q {
        input! {
            command: usize
        }
        match command {
            // 電車 x の後部と、電車 y の前部を連結させる
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                back[x] = Some(y);
                front[y] = Some(x);
            }
            // 電車 x の後部と、電車 y の前部を分離させる
            2 => {
                input! {
                    x: usize,
                    y: usize,
                }
                back[x] = None;
                front[y] = None;
            }
            // 電車 x が含まれる連結成分に属する電車の番号を、先頭から順番に全て出力する
            3 => {
                input! {
                    x: usize,
                }
                let mut current = Some(x);

                // 電車 x が連結している先頭の電車の番号を取り出す
                while front[current.unwrap()] != None {
                    current = front[current.unwrap()];
                }

                // 先頭から順に電車の番号を取り出す
                let mut answer = VecDeque::new();
                while current != None {
                    answer.push_back(current.unwrap());
                    current = back[current.unwrap()];
                }

                // 電車の件数 + 電車の番号 を半角スペース区切りで出力する
                let count = answer.len();
                answer.push_front(count);
                let result = answer.iter().map(|v| v.to_string()).collect::<Vec<_>>();
                println!("{}", result.join(" "));
            }
            _ => panic!(),
        };
    }
}
