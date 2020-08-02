use proconio::input;

fn main() {
    input! {
        (n, d): (usize, usize),
        mut points: [(i64, i64); n],
    }
    let mut result = 0;
    for point in points {
        let x = point.0;
        let y = point.1;
        let distance = (((x * x) + (y * y)) as f64).sqrt();
        if distance <= d as f64 {
            result += 1;
        }
    }
    println!("{}", result);
}
