// -*- coding:utf-8-unix -*-

use proconio::input;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

fn main() {
    input! {
        q: usize,
    }
    let mut queue = VecDeque::new();
    let mut binary_heap = BinaryHeap::new();
    let mut answers = Vec::new();

    for _ in 0..q {
        input! {
            c: usize
        }
        if c == 1 {
            input! {x: usize};
            // キューに x を pushする
            queue.push_back(x);
        } else if c == 2 {
            if binary_heap.is_empty() {
                // 優先度付きキューが空の場合、キューの先頭の要素を pop する
                let first = queue.pop_front().unwrap();
                answers.push(first);
            } else {
                // 優先度付きキューが空でない場合、優先度付きキューの最小の要素を pop する
                if let Some(Reverse(first)) = binary_heap.pop() {
                    answers.push(first);
                }
            }
        } else if c == 3 {
            // キューの要素をすべて優先度付きキューに移す
            while !queue.is_empty() {
                binary_heap.push(Reverse(queue.pop_front().unwrap()));
            }
        }
    }
    for answer in answers {
        println!("{}", answer);
    }
}
