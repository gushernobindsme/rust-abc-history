use proconio::input;

fn main() {
    input! {
        (mut x, mut k, d): (i64, i64, i64),
    }

    // 座標はマイナスからはじまることもある
    if x < 0 {
        x *= -1;
    }

    // 最低限、 x/d 分は確実に移動できるはず
    let min = x / d;

    if min < k {
        // ターン数が余っている場合
        x = x - (d * min);
        k = k - min;
        if k % 2 == 0 {
            // 偶数ならば行ったり来たりしてやり過ごせる
            println!("{}", x.abs());
        } else {
            // 奇数ならば一つ進む or 一つ戻る
            println!("{}", (x - d).abs().min((x + d).abs()));
        }
    } else {
        // ターン数を使い切れる場合
        println!("{}", x - (d * k));
    }
}
