use crate::Color::{Black, Red};
use proconio::input;
use std::str::FromStr;

// https://atcoder.jp/contests/abc173/tasks/abc173_c
fn main() {
    input! {
        (h,w,k): (usize, usize, usize),
        mut rows: [String; h],
    }

    let mut colored_list: Vec<Vec<Color>> = Vec::new();
    for row in rows {
        colored_list.push(
            row.chars()
                .into_iter()
                .map(|c| Color::from_str(c.to_string().trim()).unwrap())
                .collect(),
        );
    }

    // 行を何行か選び (0行でもよい)、列を何列か選ぶ (0 列でもよい)
    // 複数行を塗るケースをみたせていない
    let mut result = 0;
    for i in 0..=h {
        for j in 0..=w {
            let painted_list = paint(i, j, &colored_list);
            let painted_count = count(painted_list);
            // println!("i: {} j: {} black:{} result: {}", i+1, j+1, painted_count, painted_count == k);
            if painted_count == k {
                result += 1;
            }
        }
    }

    println!("{}", result);
}

fn paint(line: usize, row: usize, source: &Vec<Vec<Color>>) -> Vec<Vec<Color>> {
    let mut target: Vec<Vec<Color>> = Vec::new();

    for i in 0..source.len() {
        if line == i {
            let mut all_red: Vec<Color> = Vec::new();
            for _j in 0..source.get(i).unwrap().len() {
                all_red.push(Red);
            }
            target.push(all_red);
            continue;
        }

        let mut target_row: Vec<Color> = Vec::new();
        for j in 0..source.get(i).unwrap().len() {
            if j == row {
                target_row.push(Red);
            } else {
                let color_string = source.get(i).unwrap().get(j).unwrap().to_string();
                target_row.push(Color::from_str(&*color_string).unwrap());
            }
        }
        target.push(target_row);
    }
    target
}

fn count(target: Vec<Vec<Color>>) -> usize {
    let mut count: usize = 0;
    for row in target {
        count += row.into_iter().filter(|r| r == &Black).count();
    }
    count
}

#[derive(PartialEq, Debug)]
enum Color {
    Black,
    White,
    Red,
}

impl FromStr for Color {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Color::White),
            "#" => Ok(Color::Black),
            _ => Err(()),
        }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::White => ".",
            Color::Black => "#",
            _ => "",
        }.parse().unwrap()
    }
}
