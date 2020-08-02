use proconio::input;

fn main() {
    input! {
        x: isize,
    }
    println!("{}", if x >= 30 { "Yes" } else { "No" });
}
