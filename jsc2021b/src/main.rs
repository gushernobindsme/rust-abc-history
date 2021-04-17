// -*- coding:utf-8-unix -*-

use itertools::*;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }
    let mut list: Vec<usize> = a.to_vec();
    list.append(&mut b.to_vec());
    list.sort();

    let group_map: Vec<(usize, usize)> = list
        .into_iter()
        .group_by(|x| (*x))
        .into_iter()
        .map(|(key, group)| (key, group.count()))
        .collect::<Vec<_>>();

    let answer = group_map
        .into_iter()
        .filter(|x| x.1 == 1)
        .map(|x| x.0.to_string())
        .collect::<Vec<_>>();
    println!("{}", answer.join(" "));
}
