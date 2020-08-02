use proconio::input;

fn main() {
    input! {
        k: u64,
    }
    let mut result = 0; // 7, 77, 777 ...
    for i in 1..=k {
        result = (result * 10 + 7) % k;
        if result == 0 {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
