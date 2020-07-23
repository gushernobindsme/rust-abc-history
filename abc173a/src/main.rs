use proconio::input;

fn main() {
    input! {
        n: i32, // 購入金額
    }

    let m = n % 1000;
    println!("{}", if m == 0 { m } else { 1000 - m });
}
