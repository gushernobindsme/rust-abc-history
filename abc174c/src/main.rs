use proconio::input;
use num_traits::pow;

fn main() {
    input! {
        k: usize,
    }
    let mut result = 1; // 1, 2, 3, ...

    if k % 2 == 0 {
        println!("-1");
    } else {
        let mut seven: usize = 7; // 7, 77, 777, ...
        while seven % k != 0 {
            seven = seven + (7 * pow(10, result));
            result += 1;
        }
        println!("{}", result);
    }
}
