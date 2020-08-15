use proconio::input;

// TODO AC ならず
fn main() {
    input! {
        (mut x, k, d): (i64, u64, u64),
    }

    let mut result = x;
    for i in 0..k {
        if (x - d as i64) > 0 {
            // 座標がプラスになる場合はそのまま移動する
            x -= d as i64;
        } else if i == k - 1 {
            // 最後の手番の場合はなにもしない
            break;
        } else {
            // 座標がマイナスになる場合は引き返す
            x += d as i64;
        }
        result = x;
    }
    println!("{}", result);
}
