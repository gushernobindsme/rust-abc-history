use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
      _n: usize,
      mut c: Chars,
    };
    let red = c.iter().filter(|&s| *s == 'R').count();
    let mut result = 0;
    for i in 0..red {
        if c[i as usize] == 'W' {
            result += 1;
        }
    }
    println!("{}", result);
}
