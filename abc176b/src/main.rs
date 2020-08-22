use proconio::input;

fn main() {
    input! {
        n: String,
    }
    let result: i32 = n
        .chars()
        .into_iter()
        .map(|v| v.to_string().parse::<i32>().unwrap())
        .sum();
    if result % 9 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
