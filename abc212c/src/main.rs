// -*- coding:utf-8-unix -*-

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i32; n],
        mut b: [i32; m],
    }

    a.sort();
    b.sort();

    let ans = a
        .iter()
        .map(|&x| {
            let lb = b.lower_bound(&x);
            let mut ans = std::i32::MAX;
            if lb != 0 {
                ans = ans.min((x - b[lb - 1]).abs());
            }
            if lb != m {
                ans = ans.min((x - b[lb]).abs());
            }
            ans
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
