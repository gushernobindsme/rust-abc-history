// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: String,
    }
    let numbers = x
        .chars()
        .collect::<Vec<_>>()
        .iter()
        .map(|x| (x.to_string()).parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    if is_same(&numbers) || is_simple(&numbers) {
        println!("Weak");
    } else {
        println!("Strong");
    }
}

fn is_same(numbers: &Vec<usize>) -> bool {
    let target = numbers[0];
    for i in 1..=3 {
        if numbers[i] != target {
            return false;
        }
    }
    return true;
}

fn is_simple(numbers: &Vec<usize>) -> bool {
    let mut prev = numbers[0];
    for i in 1..=3 {
        if prev == 9 {
            if numbers[i] != 0 {
                return false;
            }
        } else if prev + 1 != numbers[i] {
            return false;
        }
        prev = numbers[i];
    }
    return true;
}
