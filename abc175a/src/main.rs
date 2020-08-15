use proconio::input;

fn main() {
    input! {
        n: String,
    }
    let mut weather: Vec<char> = Vec::new();
    for char in n.chars() {
        weather.push(char);
    }

    let mut yesterday = ' ';
    let mut result = 0;
    for i in 0..weather.len() {
        if i == 0 {
            // 1回目以降
            if weather[i] == 'R' {
                result = 1;
            }
            yesterday = weather[i];
        } else {
            // 2回目以降
            if weather[i] == 'R' && yesterday == 'R' {
                result += 1;
            } else if weather[i] == 'R' {
                result = 1;
            }
            yesterday = weather[i];
        }
    }
    println!("{}", result);
}
