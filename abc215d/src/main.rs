// -*- coding:utf-8-unix -*-

use primes::PrimeSet;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: u64,
        mut a: [u64; n],
    }
    // 集合 S に 1 〜 m までの全ての整数をいれる
    let mut s: HashSet<u64> = (1..=m).into_iter().collect();

    // 既出の素因数
    // 全体を通して同じ素因数に関して 2 度以上操作を行う必要がないことを利用して、計算量を改善する
    let mut primes = Vec::new();

    // ai 〜 an を走査する
    for ai in a {
        // ai を素因数分解する。素因数の集合を P とする
        let mut pset = PrimeSet::new();
        let p = pset.prime_factors(ai);

        // P に含まれる全ての素因数 k について、「S から k の倍数を全て削除する」という操作を行う
        for k in p {
            if primes.contains(&k) {
                continue;
            }
            let mut i = 1;
            while (k * i) <= m {
                s.remove(&(k * i));
                i = i + 1;
            }
            primes.push(k);
        }
    }

    // 操作終了後に残った整数が、解である
    let mut answers = s.into_iter().collect::<Vec<_>>();
    answers.sort();
    println!("{}", answers.len());
    for answer in answers {
        println!("{}", answer);
    }
}
