use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!(n: usize);
    input!(mut c: Chars);
    let mut result = 0;
    let mut stones: Vec<String> = Vec::new();

    for i in 0..n {
        // println!("stones: {}", stones.join(""));
        if stones.is_empty() {
            stones.push(c.get(i).unwrap().to_string());
            continue;
        }
        if c.get(i).unwrap().to_string() == String::from("W") {
            stones.push(c.get(i).unwrap().to_string());
        } else {
            if stones.get(i - 1).unwrap().to_string() == String::from("W") {
                if i == n - 1 {
                    stones.push("W".to_string());
                    result += 1;
                } else {
                    stones.push(c.get(i).unwrap().to_string());
                    stones.swap(0, i);
                    result += 1;
                }
            } else {
                stones.push(c.get(i).unwrap().to_string());
            }
        }
    }
    // println!("stones: {}", stones.join(""));
    println!("{}", result);
}
