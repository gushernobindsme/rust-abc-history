use proconio::input;

fn main() {
    input! {
        n: usize,
        mut bars: [usize; n],
    }

    let mut result = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let mut triangle = Vec::new();
                triangle.push(bars[i]);
                triangle.push(bars[j]);
                triangle.push(bars[k]);

                // 長さの異なる 3 本の棒のみを選ぶ
                triangle.sort();
                triangle.dedup_by(|a, b| a.eq(&b));
                if triangle.len() < 3 {
                    continue;
                }

                // 三角形を成立させられる場合、成功
                let len: usize = triangle.iter().map(|t| t).sum();
                let max = triangle.iter().map(|t| t).max().unwrap();
                let rest = len - max;
                if max < &rest {
                    result += 1;
                }
            }
        }
    }
    println!("{}", result);
}
