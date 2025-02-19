use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }
    let mut buf: Vec<char> = vec![];

    for c in s {
        if c == 'B' {
            buf.pop();
        } else {
            buf.push(c);
        }
    }
    let cs: String = buf.iter().collect();
    println!("{}", &cs);
}
