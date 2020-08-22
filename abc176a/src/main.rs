use proconio::input;

fn main() {
    input! {
        (n, x, t): (usize, usize, usize),
    }
    if n % x == 0 {
        // 割り切れる場合
        println!("{}", (n / x) * t);
    } else {
        // 割り切れない場合
        println!("{}", ((n / x) + 1) * t);
    }
}
