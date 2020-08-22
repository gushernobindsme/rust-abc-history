use proconio::input;

fn main() {
    input! {
        n: usize,
        mut height: [i64; n],
    }
    let mut result = 0;
    for i in 0..height.len() {
        if i == 0 {
            continue;
        } else {
            let difference = height[i - 1] - height[i];
            if difference > 0 {
                height[i] += difference; // 踏み台の分を身長に加算
                result += difference;
            }
        }
    }
    println!("{}", result);
}
