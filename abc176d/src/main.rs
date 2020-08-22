use proconio::input;

// 解けてない

fn main() {
    input! {
        (h, w): (usize, usize), // マップのサイズ
        (ch, cw): (usize, usize), // 魔法使いの出現位置
        (dh, dw): (usize, usize), // ゴール地点
        s: [proconio::marker::Chars; h] // マップ
    }

    // 魔法使いの現在位置
    let mut queue = std::collections::VecDeque::from(vec![(ch - 1, cw - 1)]);

    // 移動したことのある場所を覚えておく
    let mut dist = vec![vec![false; w]; h];
    dist[ch - 1][cw - 1] = true;

    while let Some((x, y)) = queue.pop_front() {
        // 取り出してきた状態がゴールなら探索をやめる
        if (x, y) == (dh - 1, dw - 1) {
            break;
        }
        let x = x as i32;
        let y = y as i32;
        for (i, j) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            // 移動したあとの座標を (nx, ny) とする
            let nx = x + i;
            let ny = y + j;
            if 0 <= nx
                && nx < h as i32
                && 0 <= ny
                && ny < w as i32
                && s[nx as usize][ny as usize] != '#'
                && dist[nx as usize][ny as usize] == false
            {
                // 移動できるならキューにいれる
                let nx = nx as usize;
                let ny = ny as usize;
                queue.push_back((nx, ny));
                // 移動した場所を記憶する
                dist[nx][ny] = true;
                println!("x:{} y:{}", nx, ny);
            }
        }
    }
}
