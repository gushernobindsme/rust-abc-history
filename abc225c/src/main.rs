// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut answer = true;
    let mut prev = Vec::new();
    for _ in 0..n {
        input! {
            current: [usize; m],
        }

        // 一行目のみ (i - 1) * 7 + j を満たすかをチェックする
        // 二行目以降は一つ前から -7 した値かどうかをチェックする
        if prev.is_empty() {
            // 条件に合わない場合、検証を打ち切る
            if !is_calendar(&current) {
                answer = false;
                break;
            }
        } else {
            for i in 0..m {
                // 条件に合わないものが一つでもあれば、検証を打ち切る
                if (current[i] - prev[i]) != 7 {
                    answer = false;
                    break;
                }
            }
        }
        prev = current.to_vec();
    }
    println!("{}", if answer { "Yes" } else { "No" });
}

/// (i - 1) * 7 + j を満たすか配列かをチェックする
fn is_calendar(numbers: &Vec<usize>) -> bool {
    // 7 で割ったあまりを求める
    let results = numbers.iter().map(|v| (v % 7) as i32).collect::<Vec<i32>>();

    // 正解パターンに当てはまるかをチェックする
    let mut pattern = Vec::new();
    if numbers.len() == 7 {
        pattern = vec![vec![1, 2, 3, 4, 5, 6, 0]];
    } else if numbers.len() == 6 {
        pattern = vec![vec![1, 2, 3, 4, 5, 6], vec![2, 3, 4, 5, 6, 0]];
    } else if numbers.len() == 5 {
        pattern = vec![
            vec![1, 2, 3, 4, 5],
            vec![2, 3, 4, 5, 6],
            vec![3, 4, 5, 6, 0],
        ];
    } else if numbers.len() == 4 {
        pattern = vec![
            vec![1, 2, 3, 4],
            vec![2, 3, 4, 5],
            vec![3, 4, 5, 6],
            vec![4, 5, 6, 0],
        ];
    } else if numbers.len() == 3 {
        pattern = vec![
            vec![1, 2, 3],
            vec![2, 3, 4],
            vec![3, 4, 5],
            vec![4, 5, 6],
            vec![5, 6, 0],
        ];
    } else if numbers.len() == 2 {
        pattern = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 0],
        ];
    } else if numbers.len() == 1 {
        pattern = vec![
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![5],
            vec![6],
            vec![0],
        ];
    }
    return pattern.contains(&results);
}
