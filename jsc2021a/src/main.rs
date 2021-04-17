// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: usize, // スーパー高橋での牛肉のグラム数
        y: usize, // スーパー高橋での牛肉の値段
        z: usize, // スーパーすぬけでの牛肉のグラム数
    }
    let per_price = y as f64 / x as f64;
    let price = (per_price * z as f64).ceil() as usize;
    let answer = price - 1;
    println!("{}", answer);
}
