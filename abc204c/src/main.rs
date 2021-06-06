// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::LinkedList;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut routes: [(usize, usize); m],
    }
    let mut tours = Vec::new();
    for i in 1..=n {
        let mut tour = LinkedList::new();
        tour.push_back(i);
        for route in &routes {
            let first = tour.front().unwrap();
            // グラフをつなげられる場合
            if i == *first && !tour.contains(&route.1) {
                tour.push_back(route.1);
            }
        }
        tours.push(tour);
    }
    let ans: usize = tours.iter().map(|t| t.len()).sum();
    println!("{}", ans); // この解法は WA だし TLE になる
}
